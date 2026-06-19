use common::Transport;
use rand_distr::{Distribution, Normal};
use common::StudentMeasuringParameters;

use crate::{processing::measuring::calculate_distance_between_points, structs::{depth_matrix::DepthMatrix, measurement_type::{MeasurementsType, MeasurementsTypeWithError}}};

// ------------------------------------------------------------
//  Umbrales Para Potencia y ganancia
// ------------------------------------------------------------

const DETECTION_THRESHOLD: f64 = 40.0; //db
const MAX_GAIN: f64 = 36.0; //db

// ------------------------------------------------------------
//  Otras constantes
// ------------------------------------------------------------

const SOUND_VELOCITY: f64     = 1500.0;
const TIDE_AMPLITUDE: f64     = 1.5;
const TIDE_PERIOD_H: f64      = 12.4;
const TIDE_PHASE: f64         = 0.0;

// ------------------------------------------------------------
//  Parámetros de perturbación pre-calculados
//  Se calculan una sola vez antes del loop de mediciones.
// ------------------------------------------------------------

struct DisturbanceParams {
    tide_levels: Option<Vec<f64>>,
    potency_value: f64,
    angle_std: f64,
    gain_value: f64,
}

/// Suma la distancia real recorrida por todo el path (en metros).
pub fn total_path_distance(path: &[(usize, usize)], matrix: &DepthMatrix) -> f64 {
    let mut total = 0.0;
    for i in 0..path.len().saturating_sub(1) {
        total += calculate_distance_between_points(&path[i], &path[i + 1], matrix);
    }
    total
}

/// Calcula el nivel de marea para cada punto si el alumno no usa mareógrafo.
fn calculate_tide_levels(
    n: usize,
    params: &StudentMeasuringParameters,
    path: &[(usize, usize)],
    matrix: &DepthMatrix,
) -> Option<Vec<f64>> {
    if params.transport_parameters.uses_mareograph {
        return None;
    }

    let boat_speed = params.transport_parameters.speed;
    let total_distance_m = total_path_distance(path, matrix);
    let duration_hours = total_distance_m / (boat_speed * 3600.0);

    let levels = (0..n)
        .map(|i| {
            let t = if n > 1 { (i as f64 / (n - 1) as f64) * duration_hours } else { 0.0 };
            TIDE_AMPLITUDE * (2.0 * std::f64::consts::PI * t / TIDE_PERIOD_H + TIDE_PHASE).sin()
        })
        .collect();

    Some(levels)
}

fn get_inertial_angle_std(transport: Transport, speed: f64) -> f64 {
    let base_std = match transport {
        Transport::Ship   => 1.0,
        Transport::Boat   => 3.0,
        Transport::Launch => 5.0,
    };
    let speed_factor = 1.0 + (speed / 6.17);
    base_std * speed_factor
}

fn calculate_disturbance_params(
    mediciones_len: usize,
    params: &StudentMeasuringParameters,
    path: &[(usize, usize)],
    matrix: &DepthMatrix,
) -> DisturbanceParams {
    DisturbanceParams {
        tide_levels: calculate_tide_levels(mediciones_len, params, path, matrix),
        potency_value: match params.echo_sounder_parameters.transmited_potency as u32 {
            25  => 150.0,
            50  => 200.0,
            100 => 250.0,
            _   => 200.0,
        },
        gain_value: params.echo_sounder_parameters.gain as f64,
        angle_std: get_inertial_angle_std(
            params.transport_parameters.transport,
            params.transport_parameters.speed,
        ),
    }
}

// ------------------------------------------------------------
//  Aplicación de errores
// ------------------------------------------------------------

pub fn apply_disturbances(
    mediciones: MeasurementsType,
    path: &[(usize, usize)],
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix,
) -> MeasurementsTypeWithError {
    match mediciones {
        MeasurementsType::Monohaz { measurements } => {
            let dp = calculate_disturbance_params(measurements.len(), params, path, matrix);
            MeasurementsTypeWithError::Monohaz {
                measurements: apply_disturbances_monohaz(measurements, params, matrix, &dp),
            }
        }

        MeasurementsType::Multihaz { central_measurments, paralel_measurment_1, paralel_measurment_2 } => {
            // Los tres pings son simultáneos: calculamos los parámetros una sola vez
            // usando la longitud de la lista central (todas tienen la misma cantidad).
            let dp = calculate_disturbance_params(central_measurments.len(), params, path, matrix);

            // Para multihaz aplicamos los errores en conjunto ping a ping, para que
            // central, izquierda y derecha compartan el mismo ángulo inercial y marea.
            let (central, izq, der) = apply_disturbances_multihaz(
                central_measurments,
                paralel_measurment_1,
                paralel_measurment_2,
                params,
                matrix,
                &dp,
            );

            MeasurementsTypeWithError::Multihaz {
                central_measurments: central,
                paralel_measurment_1: izq,
                paralel_measurment_2: der,
            }
        }
    }
}

// ------------------------------------------------------------
//  Monohaz: aplica errores a cada medición de forma independiente.
// ------------------------------------------------------------

fn apply_disturbances_monohaz(
    mediciones: Vec<((usize, usize), f64)>,
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix,
    dp: &DisturbanceParams,
) -> Vec<((usize, usize), Option<f64>)> {

    mediciones.into_iter().enumerate().map(|(i, (punto, p_ideal))| {
        // 1. Sensor inercial
        let (punto, p_ideal) = if params.transport_parameters.uses_inertial_sensor {
            (punto, p_ideal)
        } else {
            apply_inertial_sensor_error(punto, matrix, dp.angle_std)
        };

        let optional_p = apply_single_measurement(i, punto, p_ideal, params, matrix, dp);
        (punto, optional_p)
    }).collect()
}

// ------------------------------------------------------------
//  Multihaz: central, izquierda y derecha comparten el mismo
//  ángulo inercial y nivel de marea porque ocurren en el mismo
//  instante (mismo ping).
// ------------------------------------------------------------

fn apply_disturbances_multihaz(
    central: Vec<((usize, usize), f64)>,
    izquierda: Vec<((usize, usize), f64)>,
    derecha: Vec<((usize, usize), f64)>,
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix,
    dp: &DisturbanceParams,
) -> (
    Vec<((usize, usize), Option<f64>)>,
    Vec<((usize, usize), Option<f64>)>,
    Vec<((usize, usize), Option<f64>)>,
) {
    let mut result_central = Vec::with_capacity(central.len());
    let mut result_izq     = Vec::with_capacity(izquierda.len());
    let mut result_der     = Vec::with_capacity(derecha.len());

    for i in 0..central.len() {
        let (punto_c, _)   = central[i];
        let (punto_izq, _) = izquierda[i];
        let (punto_der, _) = derecha[i];

        // Los tres puntos del ping ocurren en el mismo instante,
        // con la misma inclinación de la embarcación.
        // Por eso muestreamos los ángulos UNA SOLA VEZ y los aplicamos a los tres.
        let (angulo_theta_x, angulo_theta_y) = if params.transport_parameters.uses_inertial_sensor {
            // Con sensor inercial no hay error de inclinación
            (0.0_f64, 0.0_f64)
        } else {
            // Sin sensor inercial: generamos los ángulos del ping
            sample_inertial_angles(dp.angle_std)
        };

        // Aplicamos los mismos ángulos a los tres puntos del ping
        let (punto_c,   p_c)   = apply_inertial_angles(punto_c,   matrix, angulo_theta_x, angulo_theta_y);
        let (punto_izq, p_izq) = apply_inertial_angles(punto_izq, matrix, angulo_theta_x, angulo_theta_y);
        let (punto_der, p_der) = apply_inertial_angles(punto_der, matrix, angulo_theta_x, angulo_theta_y);

        // Aplicamos el resto de los errores a cada punto por separado
        // (PRI, potencia, ganancia, marea — la marea usa el mismo índice i para los tres)
        result_central.push((punto_c,   apply_single_measurement(i, punto_c,   p_c,   params, matrix, dp)));
        result_izq.push(    (punto_izq, apply_single_measurement(i, punto_izq, p_izq, params, matrix, dp)));
        result_der.push(    (punto_der, apply_single_measurement(i, punto_der, p_der, params, matrix, dp)));
    }

    (result_central, result_izq, result_der)
}

// ------------------------------------------------------------
//  Lógica común para una sola medición (monohaz y multihaz).
//  Recibe la profundidad ya corregida por el sensor inercial.
// ------------------------------------------------------------

fn apply_single_measurement(
    i: usize,
    _punto: (usize, usize),
    p_ideal: f64,
    params: &StudentMeasuringParameters,
    _matrix: &DepthMatrix,
    dp: &DisturbanceParams,
) -> Option<f64> {
    let echo = &params.echo_sounder_parameters;

    // 2. PRI — si el intervalo es muy corto para la profundidad no se detecta el eco
    let t_min = 2.0 * p_ideal / SOUND_VELOCITY;
    let pri_seg = echo.pulse_repetition_interval.recip();

    // let optional_p = if pri_seg < t_min {
    //     None
    // } else {
    //     apply_power_and_gain_noise(p_ideal, dp.potency_value, dp.gain_value, echo.absortion_coefficient)
    // };
 
    // 3. Potencia y ganancia (se aplica siempre, sin el gate de PRI)
    let optional_p =
        apply_power_and_gain_noise(p_ideal, dp.potency_value, dp.gain_value, echo.absortion_coefficient);
 
    // 4. Filtro de límites
    let optional_p = apply_limits_filter(optional_p, echo.min_limit, echo.max_limit);
 
    match optional_p {
        None => None,
        Some(mut p) => {
            // 5. Velocidad del sonido
            p = if params.transport_parameters.uses_sound_profiler {
                p
            } else {
                apply_sound_velocity_noise(p, echo.sound_speed)
            };
 
            // 6. Marea — mismo nivel para central, izquierda y derecha del mismo ping
            p = if params.transport_parameters.uses_mareograph {
                p
            } else {
                apply_tide_error(p, dp.tide_levels.as_ref(), i)
            };
 
            // 7. Umbral
            p = apply_threshold_error(p, echo.threshold);
 
            Some(p)
        }
    }
}

// ------------------------------------------------------------
//  Funciones de error individuales
// ------------------------------------------------------------

/// Genera los ángulos de inclinación para un ping.
/// Se separa del aplicador para poder compartirlos entre los tres puntos del multihaz.
fn sample_inertial_angles(angle_std: f64) -> (f64, f64) {
    let normal = match Normal::new(0.0, angle_std) {
        Ok(n) => n,
        Err(_) => return (0.0, 0.0),
    };
    let mut rng = rand::rng();
    (
        normal.sample(&mut rng).to_radians(),
        normal.sample(&mut rng).to_radians(),
    )
}

/// Aplica ángulos de inclinación ya calculados a un punto.
fn apply_inertial_angles(
    punto: (usize, usize),
    matrix: &DepthMatrix,
    theta_x: f64,
    theta_y: f64,
) -> ((usize, usize), f64) {
    let p_ref = matrix.data[punto.1][punto.0];
    if p_ref <= 0.0 {
        return (punto, p_ref);
    }

    let dx = (p_ref * theta_x.tan() / matrix.size_x).round() as i64;
    let dy = (p_ref * theta_y.tan() / matrix.size_y).round() as i64;

    let x_des = (punto.0 as i64 + dx).clamp(0, matrix.width as i64 - 1) as usize;
    let y_des = (punto.1 as i64 + dy).clamp(0, matrix.height as i64 - 1) as usize;

    (punto, matrix.data[y_des][x_des])
}

/// Sin sensor inercial para monohaz: muestrea y aplica el ángulo en un solo paso.
fn apply_inertial_sensor_error(
    punto: (usize, usize),
    matrix: &DepthMatrix,
    angle_std: f64,
) -> ((usize, usize), f64) {
    let (theta_x, theta_y) = sample_inertial_angles(angle_std);
    apply_inertial_angles(punto, matrix, theta_x, theta_y)
}

fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|p| {
        if p >= min_limit && p <= max_limit { Some(p) } else { None }
    })
}

pub fn apply_power_and_gain_noise(
    p: f64,
    potency: f64,
    gain: f64,
    alpha: f64,
) -> Option<f64> {
    // 1. Pérdida de transmisión (ida y vuelta)
    let tl = 2.0 * (20.0 * p.log10() + alpha * p);

    // 2. Señal de retorno antes del amplificador
    let signal_return = potency - tl;

    // 3. Señal final post-amplificación
    let signal_final = signal_return + gain;

    if signal_final < DETECTION_THRESHOLD {
        None
    } else if gain == MAX_GAIN {
        // Eco falso: profundidad un 10% menor a la real
        Some(p * 0.9)
    } else {
        Some(p)
    }
}

fn apply_sound_velocity_noise(p: f64, v_alumno: f64) -> f64 {
    p * (v_alumno / SOUND_VELOCITY)
}

fn apply_tide_error(p: f64, tide_levels: Option<&Vec<f64>>, index: usize) -> f64 {
    match tide_levels {
        Some(levels) => p + levels[index],
        None => p,
    }
}

pub fn apply_threshold_error(p: f64, threshold_percent: f64) -> f64 {
    // 50% = correcto, sin desplazamiento
    // 10% = 10% más cerca, 90% = 10% más lejos
    let factor = (threshold_percent - 50.0) / 400.0;
    p * (1.0 + factor)
}