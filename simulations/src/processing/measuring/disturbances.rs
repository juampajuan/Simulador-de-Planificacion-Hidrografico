use rand::random;
use rand_distr::{Distribution, Normal};
use common::StudentMeasuringParameters;

use crate::{processing::measuring::calculate_distance_between_points, structs::{depth_matrix::DepthMatrix, measurement_type::{MeasurementsType, MeasurementsTypeWithError}}};

// ------------------------------------------------------------
//  Umbrales de potencia
// ------------------------------------------------------------

const HIGH_FREQ_RX_THRESHOLD: f64  = -80.0;  // dB — señal mínima que detecta el receptor
const HIGH_FREQ_SAT_THRESHOLD: f64 = 220.0;  // dB — potencia a partir de la cual hay falsos ecos

const LOW_FREQ_RX_THRESHOLD: f64   = -100.0;
const LOW_FREQ_SAT_THRESHOLD: f64  = 180.0;

// ------------------------------------------------------------
//  Umbrales de ganancia
// ------------------------------------------------------------

// Alta frecuencia: haz angosto, más sensible al ruido
const HIGH_FREQ_GAIN_LOW: f64  = 15.0;  // dB — por debajo: eco redondeado
const HIGH_FREQ_GAIN_HIGH: f64 = 35.0;  // dB — por encima: falsos ecos

// Baja frecuencia: haz más ancho, tolera más ganancia antes de amplificar ruido
const LOW_FREQ_GAIN_LOW: f64   = 10.0;
const LOW_FREQ_GAIN_HIGH: f64  = 45.0;

const MAX_LOW_GAIN_ERROR: f64  = 0.3;   // metros — error máximo por ganancia baja (OHI: ~30cm)

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
 
    mediciones.into_iter().enumerate().map(|(i, (punto, z_ideal))| {
        // 1. Sensor inercial
        let (punto, z_ideal) = if params.transport_parameters.uses_inertial_sensor {
            (punto, z_ideal)
        } else {
            apply_inertial_sensor_error(punto, matrix)
        };
 
        // 2. Velocidad del sonido
        let mut z = if params.transport_parameters.uses_sound_profiler {
            z_ideal
        } else {
            apply_sound_velocity_noise(z_ideal, echo.echosounder_velocity as f64)
        };
 
        // 3. Marea
        z = apply_tide_error(z, tide_levels.as_ref(), i);
 
        // 4. Potencia
        let z = apply_potency_noise(z, echo.transmited_potency, echo.absortion_coefficient, echo.uses_high_frecuency);
 
        // 5. Ganancia
        let z = apply_gain_noise(z, echo.gain as f64, echo.uses_high_frecuency);
 
        // 6. Filtro de límites
        let z = apply_limits_filter(z, echo.min_limit, echo.max_limit);
 
        (punto, z)
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
 
fn apply_sound_velocity_noise(z_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / SOUND_VELOCITY)
}
 
fn apply_tide_error(z_real: f64, tide_levels: Option<&Vec<f64>>, index: usize) -> f64 {
    match tide_levels {
        Some(levels) => z_real - levels[index],
        None => z_real,
    }
}
 
/// Error por potencia transmitida.
/// - Potencia insuficiente: señal no llega al receptor -> None
/// - Potencia excesiva: falsos ecos con probabilidad proporcional al exceso
fn apply_potency_noise(
    z: f64,
    transmited_potency: f64,
    absortion_coefficient: f64,
    uses_high_frecuency: bool,
) -> Option<f64> {
    let (rx_threshold, sat_threshold) = if uses_high_frecuency {
        (HIGH_FREQ_RX_THRESHOLD, HIGH_FREQ_SAT_THRESHOLD)
    } else {
        (LOW_FREQ_RX_THRESHOLD, LOW_FREQ_SAT_THRESHOLD)
    };
 
    let tl = 20.0 * z.log10() + absortion_coefficient * z;
    let p_recibida = transmited_potency - 2.0 * tl;
 
    // Señal demasiado débil
    if p_recibida < rx_threshold {
        return None;
    }
 
    // Potencia excesiva: falsos ecos superficiales
    if transmited_potency > sat_threshold {
        let exceso = transmited_potency - sat_threshold;
        let probabilidad = (exceso / sat_threshold).clamp(0.0, 1.0);
        if random::<f64>() < probabilidad {
            return Some(z * random::<f64>());
        }
    }
 
    Some(z)
}
 
/// Error por ganancia del receptor.
/// - Ganancia baja: eco redondeado -> sondaje mayor al real (hasta MAX_LOW_GAIN_ERROR)
/// - Ganancia normal: sin error
/// - Ganancia alta: amplifica ruido -> falsos ecos con probabilidad proporcional al exceso
fn apply_gain_noise(z: Option<f64>, gain: f64, uses_high_frecuency: bool) -> Option<f64> {
    let z = z?;
 
    let (gain_low, gain_high) = if uses_high_frecuency {
        (HIGH_FREQ_GAIN_LOW, HIGH_FREQ_GAIN_HIGH)
    } else {
        (LOW_FREQ_GAIN_LOW, LOW_FREQ_GAIN_HIGH)
    };
 
    if gain < gain_low {
        // Ganancia baja: error proporcional al déficit, máximo MAX_LOW_GAIN_ERROR
        let factor = 1.0 - (gain / gain_low); // 0 en el límite, 1 en gain=0
        Some(z + factor * MAX_LOW_GAIN_ERROR)
 
    } else if gain > gain_high {
        // Ganancia alta: falso eco con probabilidad proporcional al exceso
        let exceso = gain - gain_high;
        let probabilidad = (exceso / gain_high).clamp(0.0, 1.0);
        if random::<f64>() < probabilidad {
            Some(z * random::<f64>())
        } else {
            Some(z)
        }
 
    } else {
        Some(z)
    }
}
 
fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|p| {
        if p >= min_limit && p <= max_limit { Some(p) } else { None }
    })
}