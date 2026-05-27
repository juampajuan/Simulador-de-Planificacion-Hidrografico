use rand::random;
use common::{EchosounderParameters, EcosondaMode};

pub trait EchosounderLogic {
    fn create_echosounder(&mut self);
    fn apply_errors(
        &self,
        mediciones: Vec<((usize, usize), f64)>,
        v_real: f64,
        uses_sound_profiler: bool,
    ) -> Vec<((usize, usize), Option<f64>)>;
}

impl EchosounderLogic for EchosounderParameters {
    fn create_echosounder(&mut self) {
        // Usamos tu función de cálculo existente
        let (angulo_rad, alfa) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency);
        
        // Asignamos los valores reales calculados
        self.angle = angulo_rad.to_degrees();
        self.absortion_coefficient = alfa;
    }
    fn apply_errors(
        &self,
        mediciones: Vec<((usize, usize), f64)>,
        v_real: f64,
        uses_sound_profiler: bool,
    ) -> Vec<((usize, usize), Option<f64>)> {
        mediciones.into_iter().map(|(punto, z_ideal)| {
            let z = if uses_sound_profiler { z_ideal } 
                    else { apply_velocity_error(z_ideal, v_real, self.echosounder_velocity as f64) };

            // Usamos self.absortion_coefficient que ya fue calculado en create_echosounder
            let z = apply_potency_and_gain_error(z, self.transmited_potency, self.gain as f64, self.absortion_coefficient);

            let z = apply_limits_filter(z, self.min_limit, self.max_limit);
            (punto, z)
        }).collect()
    }
}

/// Calcula el angulo del haz en radianes y el coeficiente de absorcion segun frecuencia.
/// true  -> alta frecuencia -> 200 kHz, D=10cm -> φ ~4.5°,  α = 0.060 dB/m
/// false -> baja frecuencia -> 12 kHz,  D=20cm -> φ ~18°,   α = 0.004 dB/m
fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool) -> (f64, f64) {
    let v_real = 1500.0;
    let (f, d, absortion_coefficient) = if uses_high_frecuency {
        (200_000.0, 0.10, 0.060) // Alta frecuencia
    } else {
        (12_000.0, 0.20, 0.004)  // Baja frecuencia
    };
    let angulo_grados: f64 = 60.0 * (v_real / f) / d;
    (angulo_grados, absortion_coefficient)
}

fn apply_velocity_error(z_real: f64, v_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / v_real)
}

fn apply_potency_and_gain_error(
    z: f64,
    transmited_potency: f64,
    gain: f64,
    absortion_coefficient: f64,
) -> Option<f64> {
    let umbral_receptor: f64 = -100.0; // Valor de ejemplo, ajusta según necesidad
    let umbral_saturacion: f64 = 1000.0;
    let k_sensibilidad: f64 = 0.01;

    // Pérdida de transmisión ida y vuelta
    let tl = 20.0 * z.log10() + absortion_coefficient * z;
    let p_recibida = transmited_potency - 2.0 * tl;
    let p_amplificada = p_recibida + gain;

    if p_amplificada < umbral_receptor {
        return None;
    }

    if transmited_potency > umbral_saturacion {
        let probability = ((transmited_potency - umbral_saturacion) / umbral_saturacion).clamp(0.0, 1.0);
        if random::<f64>() < probability {
            return Some(z * 2.0);
        }
    }

    let gain_optima = 2.0 * tl;
    let delta_db = gain - gain_optima;
    let z_obs = z - k_sensibilidad * delta_db;

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