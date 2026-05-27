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
        // 1. El chequeo principal es si usa Monohaz o Multihaz
        self.mode = match self.uses_monohaz {
            true => {
                let angle = calculate_angle(self.uses_high_frecuency);
                Some(EcosondaMode::Monohaz { angle: angle })
            },
            false => {
                Some(EcosondaMode::Multihaz)
            }
        };
    }

}

pub enum Boat {
    Small { speed: f64, balance_index: usize },
    Medium { speed: f64, balance_index: usize},
    Large { speed: f64, balance_index: usize}
}

fn calculate_angle(uses_high_frecuency: bool) -> f64 {
    // true → alta frecuencia → 200 kHz, D=10cm → φ ~4.5°
    // false → baja frecuencia → 12 kHz, D=20cm → φ ~18°
    let v_real = 1500.0;
    let (f, d) = if uses_high_frecuency {
        (200_000.0, 0.10)
    } else {
        (12_000.0, 0.20)
    };
    60.0 * (v_real / f) / d
}