use rand::random;
use common::StudentMeasuringParameters;

use crate::{processing::measuring::calculate_distance_between_points, structs::depth_matrix::DepthMatrix};

// ------------------------------------------------------------
//  Umbrales de frecuencia Alta
// ------------------------------------------------------------

const HIGH_FREQ_RX_THRESHOLD: f64  = -80.0; // Umbral mínimo del receptor en dB
const HIGH_FREQ_SAT_THRESHOLD: f64 = 220.0; // Umbral de saturación (eco doble) en dB

// ------------------------------------------------------------
//  Umbrales de frecuencia Baja
// ------------------------------------------------------------

const LOW_FREQ_RX_THRESHOLD: f64   = -100.0; // Umbral mínimo del receptor en dB
const LOW_FREQ_SAT_THRESHOLD: f64  = 180.0;  // Umbral de saturación (eco doble) en dB

// ------------------------------------------------------------
//  Constante De Error Para Gain
// ------------------------------------------------------------

const GAIN_SENSITIVITY: f64    = 0.01;   // Metros de error por dB de desviación en ganancia

// ------------------------------------------------------------
//  Parametros generales del area a relevar
// ------------------------------------------------------------

const SOUND_VELOCITY: f64 = 1500.0; // Velocidad del sonido en el agua en m/s
const TIDE_AMPLITUDE: f64 = 1.5;  // metros
const TIDE_PERIOD_H: f64 = 12.4; // horas (semidiurna)
const TIDE_PHASE: f64 = 0.0;  // radianes


/// Suma la distancia real recorrida por todo el path (en metros).
/// Se usa para estimar la duración total del relevamiento.
pub fn total_path_distance(path: &Vec<(usize, usize)>, matrix: &DepthMatrix) -> f64 {
    let mut total_distance = 0.0;

    // Recorremos desde el primer elemento hasta el penúltimo
    for i in 0..path.len().saturating_sub(1) {
        let p1 = &path[i];
        let p2 = &path[i + 1];
        total_distance += calculate_distance_between_points(p1, p2, matrix);
    }

    total_distance
}

/// Calcula los niveles de marea si el parámetro lo requiere.
fn calculate_tide_levels(n: usize, params: &StudentMeasuringParameters,path: &Vec<(usize, usize)>, matrix: &DepthMatrix) -> Option<Vec<f64>> {
    if !params.uses_mathegapher {
        return None;
    }

    let boat_speed = match params.boat {
        common::Boat::W { speed } => speed,
        common::Boat::Y { speed } => speed,
    };

    let total_distance_m = total_path_distance(path, matrix);
    let duration_hours =  total_distance_m / (boat_speed * 3600.0);

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

pub fn apply_errors(
    mediciones: Vec<((usize, usize), f64)>,
    path: &Vec<(usize, usize)>,
    params: &StudentMeasuringParameters,
    matrix: &DepthMatrix
) -> Vec<((usize, usize), Option<f64>)> {
    let echo = &params.echo_sounder_parameters;

    let tide_levels= calculate_tide_levels(mediciones.len(), params, path, matrix);

    mediciones.into_iter().enumerate().map(|(i, (punto, z_ideal))| {
        let mut z = if params.uses_sound_profiler{
            z_ideal
        } else {
            apply_velocity_error(z_ideal, echo.echosounder_velocity as f64)
        };

        z = apply_tide_error(z, tide_levels.as_ref(), i);

        let z_optional= apply_potency_and_gain_error(
            z,
            echo.transmited_potency,
            echo.gain as f64,
            echo.absortion_coefficient,
            echo.uses_high_frecuency
        );

        apply_limits_filter(z_optional, echo.min_limit, echo.max_limit);

        (punto, z_optional)
    }).collect()
}



fn apply_velocity_error(z_real: f64,v_alumno: f64) -> f64 {
    z_real * (v_alumno / SOUND_VELOCITY)
}

fn apply_tide_error(z_real: f64, tide_levels: Option<&Vec<f64>>, index: usize) -> f64 {
    match tide_levels {
        Some(levels) => z_real - levels[index],
        None => z_real,
    }
}

fn apply_potency_and_gain_error(
    z: f64,
    transmited_potency: f64,
    gain: f64,
    absortion_coefficient: f64,
    uses_high_frecuency: bool,
) -> Option<f64> {
    let (umbral_receptor, umbral_saturacion) = if uses_high_frecuency {
        (HIGH_FREQ_RX_THRESHOLD, HIGH_FREQ_SAT_THRESHOLD)
    } else {
        (LOW_FREQ_RX_THRESHOLD, LOW_FREQ_SAT_THRESHOLD)
    };

    let tl = 20.0 * z.log10() + absortion_coefficient * z;
    let p_recibida = transmited_potency - 2.0 * tl;
    let p_amplificada = p_recibida + gain;

    if p_amplificada < umbral_receptor {
        return None; // Señal no llega: sin medición
    }

    if transmited_potency > umbral_saturacion {
        let probability = ((transmited_potency - umbral_saturacion) / umbral_saturacion)
            .clamp(0.0, 1.0);
        if random::<f64>() < probability {
            return Some(z * 2.0); // Eco doble
        }
    }

    // Ganancia óptima = compensar exactamente las pérdidas (delta_db = 0 → sin error)
    // delta_db > 0 (ganancia alta) → z_obs < z_real
    // delta_db < 0 (ganancia baja) → z_obs > z_real  (eco "redondeado")
    let gain_optima = 2.0 * tl;
    let delta_db = gain - gain_optima;
    let z_obs = z - GAIN_SENSITIVITY * delta_db;

    Some(z_obs)
}

fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|p| {
        if p >= min_limit && p <= max_limit { Some(p) } else { None }
    })
}