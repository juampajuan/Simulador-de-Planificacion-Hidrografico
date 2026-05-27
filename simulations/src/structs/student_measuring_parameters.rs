use rand::random;
use crate::structs::echosonder::EcosondaMode;

pub struct StudentMeasuringParameters {
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub echo_sounder_parameters: EchosounderParameters,
    pub boat: Boat
}

pub struct EchosounderParameters {
    pub uses_monohaz: bool,
    pub mode: Option<EcosondaMode>,
    pub max_limit: f64,
    pub min_limit: f64,
    pub pulse_repetition_interval: f64, //ms
    pub pulse_length: usize,
    pub uses_high_frecuency: bool,
    pub transmited_potency: f64,
    pub gain: f32,
    pub echosounder_velocity: usize,
    pub threshold: f64,
}

impl EchosounderParameters {

    pub fn create_echosounder(&mut self) {
        self.mode = match self.uses_monohaz {
            true => {
                // angulo en radianes para que tan() funcione correctamente
                let (angle,absortion_coefficient) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency);
                Some(EcosondaMode::Monohaz { angle, absortion_coefficient})
            },
            false => {
                Some(EcosondaMode::Multihaz)
            }
        };
    }

    /// Pipeline de errores.
    /// Recibe mediciones ideales como (punto, z_ideal)
    /// Devuelve (punto, Option<f64>):
    ///   Some(z) -> medicion valida con errores aplicados
    ///   None    -> medicion perdida
    ///
    /// uses_sound_profiler: si true, no se aplica error de velocidad
    pub fn apply_errors(
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
                    z_ideal // perfilador mide v_real correctamente -> sin error
                } else {
                    apply_velocity_error(z_ideal, v_real, self.echosounder_velocity as f64)
                };

                let z = apply_potency_error(z, self.transmited_potency, &self.mode);

                // 2. Filtro por limites min/max
                let z = apply_limits_filter(z, self.min_limit, self.max_limit);

                (punto, z)
            })
            .collect()
    }
}

//velociades del bote en metros/ms
pub struct Boat {
    pub speed: f64, //metros/ms
    pub balance_index: usize,
}

fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool) -> (f64,f64) {
    // true  -> alta frecuencia -> 200 kHz, D=10cm -> φ ~4.5° - absortio coefficient = aprox 0.004 dB/m
    // false -> baja frecuencia -> 12 kHz,  D=20cm -> φ ~18° - absortio coefficient = aprox 0.060 dB/m
    let v_real = 1500.0;
    let absortion_coefficient: f64;
    let (f, d) = if uses_high_frecuency {
        absortion_coefficient = 0.060;
        (200_000.0, 0.10)
    } else {
        absortion_coefficient = 0.004;
        (12_000.0, 0.20)
    };
    let angulo_grados:f64 = 60.0 * (v_real / f) / d;

    (angulo_grados.to_radians(),absortion_coefficient)
}

/// Error por velocidad del sonido mal configurada.
/// Formula simplificada (b=0, k=0, m=0):
///   z_obs = z_real * (v_alumno / v_real)
fn apply_velocity_error(z_real: f64, v_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / v_real)
}

// Si la potencia es muy baja no detecta nada, si supera el umbral, hay probabilidad de ecos repetidos
// Cuando tengamos los parametros del profesor tenemos que reemplazar el umbral harcodeado
fn apply_potency_error(z:f64, transmited_potency: f64, mode :&Option<EcosondaMode>)-> Option<f64>{
    let umbral_min = f64::MIN;
    let umbral_max = f64::MIN;

    let absortion_coefficient = match mode {
        Some(EcosondaMode::Monohaz { absortion_coefficient, .. }) => *absortion_coefficient,
        _ => 0.0,
    };

    let potency_loss = 20.0*z.log10() + absortion_coefficient*z;

    let recieved_potency = transmited_potency - 2.0*potency_loss;

    if (recieved_potency >= umbral_min) && (recieved_potency <= umbral_max){
        Some(z)
    } else if recieved_potency > umbral_max{
        let probability_false_echo = ((transmited_potency - umbral_max) / umbral_max).clamp(0.0, 1.0);

        let _new_z = z;

        if random::<f64>() < probability_false_echo {
            let _new_z = z*2.0; // eco fantasma
        }

        Some(_new_z)
    } else {
        None
    }
}

/// Filtra mediciones fuera del rango [min_limit, max_limit].
fn apply_limits_filter(z: Option<f64>, min_limit: f64, max_limit: f64) -> Option<f64> {
    z.and_then(|profundidad| {
        if profundidad >= min_limit && profundidad <= max_limit {
            Some(profundidad)
        } else {
            None
        }
    })
}

