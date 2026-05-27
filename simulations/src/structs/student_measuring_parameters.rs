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
        self.mode = match self.uses_monohaz {
            true => {
                let (angle, absortion_coefficient) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency);
                Some(EcosondaMode::Monohaz { angle, absortion_coefficient })
            },
            false => {
                Some(EcosondaMode::Multihaz)
            }
        };
    }

    fn apply_errors(
        &self,
        mediciones: Vec<((usize, usize), f64)>,
        v_real: f64,
        uses_sound_profiler: bool,
    ) -> Vec<((usize, usize), Option<f64>)> {
        mediciones
            .into_iter()
            .map(|(punto, z_ideal)| {
                // 1. Error por velocidad del sonido
                let z = if uses_sound_profiler {
                    z_ideal
                } else {
                    apply_velocity_error(z_ideal, v_real, self.echosounder_velocity as f64)
                };

                // 2. Error por potencia y ganancia (modelo unificado)
                let z = apply_potency_and_gain_error(z, self.transmited_potency, self.gain as f64, &self.mode);

                // 3. Filtro por limites min/max
                let z = apply_limits_filter(z, self.min_limit, self.max_limit);

                (punto, z)
            })
            .collect()
    }
}

/// Calcula el angulo del haz en radianes y el coeficiente de absorcion segun frecuencia.
/// true  -> alta frecuencia -> 200 kHz, D=10cm -> φ ~4.5°,  α = 0.060 dB/m
/// false -> baja frecuencia -> 12 kHz,  D=20cm -> φ ~18°,   α = 0.004 dB/m
fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool) -> (f64, f64) {
    let v_real = 1500.0;
    let (f, d, absortion_coefficient) = if uses_high_frecuency {
        (200_000.0, 0.10, 0.060)
    } else {
        (12_000.0, 0.20, 0.004)
    };
    let angulo_grados: f64 = 60.0 * (v_real / f) / d;
    (angulo_grados.to_radians(), absortion_coefficient)
}

/// Error por velocidad del sonido mal configurada.
/// Formula simplificada (b=0, k=0, m=0):
///   z_obs = z_real * (v_alumno / v_real)
fn apply_velocity_error(z_real: f64, v_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / v_real)
}

fn apply_potency_and_gain_error(
    z: f64,
    transmited_potency: f64,
    gain: f64,
    mode: &Option<EcosondaMode>,
) -> Option<f64> {

    // TODO: Fernando debe definir estos valores segun el equipo
    let umbral_receptor: f64 = f64::MIN;    // dB: minimo para detectar el eco
    let umbral_saturacion: f64 = f64::MAX;  // dB: maximo antes de ecos fantasma
    let k_sensibilidad: f64 = 0.01;         // metros por dB de desviacion del optimo

    let absortion_coefficient = match mode {
        Some(EcosondaMode::Monohaz { absortion_coefficient, .. }) => *absortion_coefficient,
        _ => 0.0,
    };

    // Perdida de transmision ida y vuelta
    let tl = 20.0 * z.log10() + absortion_coefficient * z;
    let p_recibida = transmited_potency - 2.0 * tl;
    let p_amplificada = p_recibida + gain;

    // Eco demasiado debil -> medicion perdida
    if p_amplificada < umbral_receptor {
        return None;
    }

    // Saturacion por potencia excesiva -> probabilidad de eco fantasma
    if transmited_potency > umbral_saturacion {
        let probability_false_echo = ((transmited_potency - umbral_saturacion) / umbral_saturacion).clamp(0.0, 1.0);
        if random::<f64>() < probability_false_echo {
            return Some(z * 2.0); // eco fantasma: segundo rebote
        }
    }

    // Por ultimo: medicion valida pero con error si gain != gain_optima
    // gain_optima compensa exactamente la perdida: gain_optima = 2 * TL
    let gain_optima = 2.0 * tl;
    let delta_db = gain - gain_optima;

    // delta > 0 -> sobrecompensado -> campana inflada -> umbral cruzado antes -> parece menos profundo
    // delta < 0 -> subcompensado   -> campana aplastada -> umbral cruzado tarde -> parece mas profundo
    let z_obs = z - k_sensibilidad * delta_db;

    Some(z_obs)
}

/// Filtra mediciones fuera del rango [min_limit, max_limit].
/// Devuelve None si cae fuera -> medicion perdida.
fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|profundidad| {
        if profundidad >= min_limit && profundidad <= max_limit {
            Some(profundidad)
        } else {
            None
        }
    })
}