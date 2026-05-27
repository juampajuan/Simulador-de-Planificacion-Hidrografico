use crate::EcosondaMode;

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
    pub pulse_repetition_interval: usize,
    pub pulse_length: usize,
    pub uses_high_frecuency: bool,
    pub transmited_potency: f64,
    pub gain: f32,
    pub echosounder_velocity: usize,
}

impl EchosounderParameters {

    pub fn create_echosounder(&mut self) {
        self.mode = match self.uses_monohaz {
            true => {
                // angulo en radianes para que tan() funcione correctamente
                let angle = calculate_angle_radians(self.uses_high_frecuency);
                Some(EcosondaMode::Monohaz { angle })
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

                // 2. Filtro por limites min/max
                let z = apply_limits_filter(z, self.min_limit, self.max_limit);

                (punto, z)
            })
            .collect()
    }
}

pub enum Boat {
    Small { speed: f64, balance_index: usize },
    Medium { speed: f64, balance_index: usize},
    Large { speed: f64, balance_index: usize}
}

fn calculate_angle_radians(uses_high_frecuency: bool) -> f64 {
    // true  -> alta frecuencia -> 200 kHz, D=10cm -> φ ~4.5°
    // false -> baja frecuencia -> 12 kHz,  D=20cm -> φ ~18°
    let v_real = 1500.0;
    let (f, d) = if uses_high_frecuency {
        (200_000.0, 0.10)
    } else {
        (12_000.0, 0.20)
    };
    let angulo_grados:f64 = 60.0 * (v_real / f) / d;
    angulo_grados.to_radians()
}

/// Error por velocidad del sonido mal configurada.
/// Formula simplificada (b=0, k=0, m=0):
///   z_obs = z_real * (v_alumno / v_real)
fn apply_velocity_error(z_real: f64, v_real: f64, v_alumno: f64) -> f64 {
    z_real * (v_alumno / v_real)
}

/// Filtra mediciones fuera del rango [min_limit, max_limit].
/// Devuelve None si cae fuera -> medicion perdida.
fn apply_limits_filter(z: f64, min_limit: f64, max_limit: f64) -> Option<f64> {
    if z >= min_limit && z <= max_limit {
        Some(z)
    } else {
        None
    }
}