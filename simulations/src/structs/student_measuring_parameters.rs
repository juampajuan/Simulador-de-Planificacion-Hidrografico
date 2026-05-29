use rand::random;
use common::{EchosounderParameters, EcosondaMode};

// ------------------------------------------------------------
//  Constantes físicas — Alta frecuencia (200 kHz, D=10cm)
// ------------------------------------------------------------

const HIGH_FREQ_HZ: f64        = 200000.0; // Frecuencia en Hz
const HIGH_FREQ_DIAMETER: f64  = 0.10;      // Diámetro del transductor en metros
const HIGH_FREQ_ALPHA: f64     = 0.060;     // Coeficiente de absorción en dB/m
const HIGH_FREQ_RX_THRESHOLD: f64  = -80.0; // Umbral mínimo del receptor en dB
const HIGH_FREQ_SAT_THRESHOLD: f64 = 220.0; // Umbral de saturación (eco doble) en dB

// ------------------------------------------------------------
//  Constantes físicas — Baja frecuencia (12 kHz, D=20cm)
// ------------------------------------------------------------

const LOW_FREQ_HZ: f64         = 12000.0;  // Frecuencia en Hz
const LOW_FREQ_DIAMETER: f64   = 0.20;      // Diámetro del transductor en metros
const LOW_FREQ_ALPHA: f64      = 0.004;     // Coeficiente de absorción en dB/m
const LOW_FREQ_RX_THRESHOLD: f64   = -100.0; // Umbral mínimo del receptor en dB
const LOW_FREQ_SAT_THRESHOLD: f64  = 180.0;  // Umbral de saturación (eco doble) en dB

// ------------------------------------------------------------
//  Constantes del modelo de error
// ------------------------------------------------------------

const SOUND_VELOCITY: f64      = 1500.0; // Velocidad del sonido en el agua en m/s
const BEAM_WIDTH_FACTOR: f64   = 60.0;   // Factor para cálculo de ancho del haz (Clase 03b)
const GAIN_SENSITIVITY: f64    = 0.01;   // Metros de error por dB de desviación en ganancia

// ------------------------------------------------------------
//  Trait público
// ------------------------------------------------------------

pub trait EchosounderLogic {
    fn create_echosounder(&mut self);
    fn apply_errors(
        &self,
        mediciones: Vec<((usize, usize), f64)>,
        v_real: f64,
        uses_sound_profiler: bool,
    ) -> Vec<((usize, usize), Option<f64>)>;
}

// ------------------------------------------------------------
//  Implementación del trait
// ------------------------------------------------------------

impl EchosounderLogic for EchosounderParameters {
    fn create_echosounder(&mut self) {
        let (angulo_rad, alfa) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency);
        self.angle = angulo_rad;
        self.absortion_coefficient = alfa;
    }

    fn apply_errors(
        &self,
        mediciones: Vec<((usize, usize), f64)>,
        v_real: f64,
        uses_sound_profiler: bool,
    ) -> Vec<((usize, usize), Option<f64>)> {
        mediciones.into_iter().map(|(punto, z_ideal)| {
            let z = if uses_sound_profiler {
                z_ideal
            } else {
                apply_velocity_error(z_ideal, v_real, self.echosounder_velocity as f64)
            };

            let z = apply_potency_and_gain_error(
                z,
                self.transmited_potency,
                self.gain as f64,
                self.absortion_coefficient,
                self.uses_high_frecuency,
            );

            let z = apply_limits_filter(z, self.min_limit, self.max_limit);
            (punto, z)
        }).collect()
    }
}

// ------------------------------------------------------------
//  Cálculo de parámetros físicos de la ecosonda
// ------------------------------------------------------------


fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool) -> (f64, f64) {
    let (frecuencia, diametro, alfa) = if uses_high_frecuency {
        (HIGH_FREQ_HZ, HIGH_FREQ_DIAMETER, HIGH_FREQ_ALPHA)
    } else {
        (LOW_FREQ_HZ, LOW_FREQ_DIAMETER, LOW_FREQ_ALPHA)
    };
    let angulo_grados: f64 = BEAM_WIDTH_FACTOR * (SOUND_VELOCITY / frecuencia) / diametro;
    (angulo_grados, alfa)
}

// ------------------------------------------------------------
//  Aplicación de errores
// ------------------------------------------------------------

fn apply_velocity_error(z_real: f64, v_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / v_real)
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
    z.and_then(|profundidad| {
        if profundidad >= min_limit && profundidad <= max_limit {
            Some(profundidad)
        } else {
            None
        }
    })
}