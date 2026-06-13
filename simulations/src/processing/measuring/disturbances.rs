use rand::random;
use rand::RngExt;
use rand_distr::{Distribution, Normal};
use common::StudentMeasuringParameters;

use crate::{processing::measuring::calculate_distance_between_points, structs::{depth_matrix::DepthMatrix, measurement_type::{MeasurementsType, MeasurementsTypeWithError}}};

// ------------------------------------------------------------
//  Umbrales de potencia
// ------------------------------------------------------------

const HIGH_FREQ_POTENCY_THRESHOLD: f64  = 70.0; //en %
const LOW_FREQ_POTENCY_THRESHOLD: f64 = 30.0; //en %

// ------------------------------------------------------------
//  Umbrales de ganancia
// ------------------------------------------------------------

const HIGH_FREQ_GAIN_THRESHOLD: f64  = 30.0; // en db
const LOW_FREQ_GAIN_FALSE_ECO_THRESHOLD: f64  = 30.0; // en db
const LOW_FREQ_GAIN_THRESHOLD: f64  = 15.0; // en db



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
            apply_sound_velocity_noise(z_ideal, params.echo_sounder_parameters.sound_speed)
        };
 
        // 3. Marea
        z = apply_tide_error(z, tide_levels.as_ref(), i);
 
        // 4. Potencia
        let z = apply_potency_noise(z, echo.transmited_potency, echo.uses_high_frecuency);
 
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
        Some(levels) => z_real + levels[index],
        None => z_real,
    }
}
 
/// Error por potencia transmitida.
/// - Potencia insuficiente: señal no llega al receptor -> None
/// - Potencia excesiva: falsos ecos con probabilidad proporcional al exceso
fn apply_potency_noise(
    z: f64,
    transmited_potency: f64,
    uses_high_frecuency: bool,
) -> Option<f64> {

    
    let threshold = if uses_high_frecuency {
        HIGH_FREQ_POTENCY_THRESHOLD
    } else {
        LOW_FREQ_POTENCY_THRESHOLD    
    };

    if transmited_potency < threshold {
        let excess =  threshold - transmited_potency;
        let false_echo_prob = excess / threshold; // Probabilidad proporcional al exceso
        if random::<f64>() < false_echo_prob {
            None // no se detecta el eco real
        } else {
            Some(z)
        }
    } else {
        Some(z)
    }
}
 
fn apply_gain_noise(z: Option<f64>, gain: f64, uses_high_frecuency: bool) -> Option<f64> {
    if uses_high_frecuency == true {
        if gain > HIGH_FREQ_GAIN_THRESHOLD {
            z
        } else {
            z.and_then(|p| {
                let excess =  HIGH_FREQ_GAIN_THRESHOLD - gain;
                let echo_didnt_retorn_prob = excess / HIGH_FREQ_GAIN_THRESHOLD; // Probabilidad proporcional al exceso
                if random::<f64>() < echo_didnt_retorn_prob {
                    None // no se detecta el eco real
                } else {
                    Some(p)
                }
            })
        }
    } else {
        if gain > LOW_FREQ_GAIN_THRESHOLD {
            if gain > LOW_FREQ_GAIN_FALSE_ECO_THRESHOLD {
                let excess = gain - LOW_FREQ_GAIN_FALSE_ECO_THRESHOLD;
                let false_echo_prob = excess / LOW_FREQ_GAIN_FALSE_ECO_THRESHOLD; // Probabilidad proporcional al exceso
                if random::<f64>() < false_echo_prob {
                    // eco falso: profundidad menor a la real, con algo de aleatoriedad
                    let reduction =  rand::rng().random_range(0.1..0.5); // 10% a 50% menos
                    z.and_then(|p| {Some(p * (1.0 - reduction))}) // no se detecta el eco real
                } else {
                    z
                }
            } else {
                z
            }
        } else {
            z.and_then(|p| {
                let excess = gain - LOW_FREQ_GAIN_THRESHOLD;
                let echo_didnt_retorn_prob = excess / LOW_FREQ_GAIN_THRESHOLD; // Probabilidad proporcional al exceso
                if random::<f64>() < echo_didnt_retorn_prob {
                    None
                } else {
                    Some(p)
                }
            })
        }
    }

}
 
fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|p| {
        if p >= min_limit && p <= max_limit { Some(p) } else { None }
    })
}