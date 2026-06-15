use rand::random;
use rand::RngExt;
use rand_distr::{Distribution, Normal};
use common::StudentMeasuringParameters;

use crate::{processing::measuring::calculate_distance_between_points, structs::{depth_matrix::DepthMatrix, measurement_type::{MeasurementsType, MeasurementsTypeWithError}}};

// ------------------------------------------------------------
//  Umbrales Para Potencia y ganancia
// ------------------------------------------------------------

const DETECTION_THRESHOLD: f64 = 40.0; //db
const SATURATION_THRESHOLD: f64 = 220.0; //db

// ------------------------------------------------------------
//  Otras constantes
// ------------------------------------------------------------

const SOUND_VELOCITY: f64         = 1500.0;
const TIDE_AMPLITUDE: f64         = 1.5;
const TIDE_PERIOD_H: f64          = 12.4;
const TIDE_PHASE: f64             = 0.0;
const INERTIAL_ANGLE_STD_DEG: f64 = 3.0;


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
 
// ------------------------------------------------------------
//  Aplicación de errores
// ------------------------------------------------------------
 
pub fn apply_disturbances_monohaz(
    mediciones: Vec<((usize, usize), f64)>,
    path: &[(usize, usize)],
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix,
) -> Vec<((usize, usize), Option<f64>)> {
    let echo = &params.echo_sounder_parameters;
    let tide_levels = calculate_tide_levels(mediciones.len(), params, path, matrix);
 
    mediciones.into_iter().enumerate().map(|(i, (punto, p_ideal))| {
        // 1. Sensor inercial
        let (punto, p_ideal) = if params.transport_parameters.uses_inertial_sensor {
            (punto, p_ideal)
        } else {
            apply_inertial_sensor_error(punto, matrix)
        };

        // 2. Potencia y ganancia
        let optional_p = apply_power_and_gain_noise(p_ideal, echo.transmited_potency, echo.gain, echo.absortion_coefficient);

        // 3. Filtro de límites
        let optional_p = apply_limits_filter(optional_p, echo.min_limit, echo.max_limit);
        
        match optional_p {
            None => (),
            Some(mut p) => {

                    // 4. Velocidad del sonido
                    p = if params.transport_parameters.uses_sound_profiler {
                        p
                    } else {
                        apply_sound_velocity_noise(p, params.echo_sounder_parameters.sound_speed)
                    };
            
                    // 5. Marea
                    apply_tide_error(p, tide_levels.as_ref(), i);
                }
 
        }

        (punto, optional_p)
        
    }).collect()
}


pub fn apply_disturbances(
    mediciones: MeasurementsType,
    path: &[(usize, usize)],
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix,
) -> MeasurementsTypeWithError {
    match mediciones {
        MeasurementsType::Monohaz { measurements } => {
            MeasurementsTypeWithError::Monohaz {measurements: apply_disturbances_monohaz(measurements, path, params, matrix)}
        }
    
        MeasurementsType::Multihaz { central_measurments, paralel_measurment_1, paralel_measurment_2 } => {
            MeasurementsTypeWithError::Multihaz {
                central_measurments: apply_disturbances_monohaz(central_measurments, path, params, matrix),
                paralel_measurment_1: apply_disturbances_monohaz(paralel_measurment_1, path, params, matrix),
                paralel_measurment_2: apply_disturbances_monohaz(paralel_measurment_2, path, params, matrix),
            }
        }
    }
}
 
// ------------------------------------------------------------
//  Funciones de error individuales
// ------------------------------------------------------------
 
/// Sin sensor inercial: el barco se inclina (roll + pitch gaussiano σ=3°).
/// El haz ilumina un punto desplazado pero se asigna a la coordenada vertical.
fn apply_inertial_sensor_error(
    punto: (usize, usize),
    matrix: &DepthMatrix,
) -> ((usize, usize), f64) {
    let normal = if let Ok(n) = Normal::new(0.0, INERTIAL_ANGLE_STD_DEG) {
        n
    } else {
        return (punto, matrix.data[punto.1][punto.0]);
    };
 
    let mut rng = rand::rng();
    let theta_x = normal.sample(&mut rng).to_radians();
    let theta_y = normal.sample(&mut rng).to_radians();
 
    let z_ref = matrix.data[punto.1][punto.0];
    if z_ref <= 0.0 {
        return (punto, z_ref);
    }
 
    let dx = (z_ref * theta_x.tan() / matrix.size_x).round() as i64;
    let dy = (z_ref * theta_y.tan() / matrix.size_y).round() as i64;
 
    let x_des = (punto.0 as i64 + dx).clamp(0, matrix.width as i64 - 1) as usize;
    let y_des = (punto.1 as i64 + dy).clamp(0, matrix.height as i64 - 1) as usize;
 
    (punto, matrix.data[y_des][x_des])
}
 
fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|p| {
        if p >= min_limit && p <= max_limit { Some(p) } else { None }
    })
}

pub fn apply_power_and_gain_noise(
    p: f64,
    potency: f64,      // 150.0, 200.0 o 250.0
    gain: f64,         // 12.0, 24.0 o 36.0
    alpha: f64,        // 0.004 o 0.06 según frecuencia
) -> Option<f64> {

    // 1. Pérdida de transmisión (ida y vuelta)
    let tl = 2.0 * (20.0 * p.log10() + alpha * p);

    // 2. Señal de retorno antes del amplificador
    let signal_return = potency - tl;

    // 3. Señal final post-amplificación
    let signal_final = signal_return + gain;

    if signal_final < DETECTION_THRESHOLD {
        // Eco perdido: señal demasiado débil
        None
    } else if signal_final > SATURATION_THRESHOLD {
        // Eco falso: saturación proporcional al exceso
        // A mayor exceso sobre el umbral, más se reduce la profundidad medida
        let excess = signal_final - SATURATION_THRESHOLD;
        let max_excess = 100.0; // exceso máximo esperado para normalizar
        let reduction = (excess / max_excess).min(0.5); // máximo 50% de reducción
        Some(p * (1.0 - reduction))
    } else {
        // Lectura correcta
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