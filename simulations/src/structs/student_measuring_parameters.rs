pub struct StudentMeasuringParameters {
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub echo_sounder_parameters: EchosounderParameters,
    pub boat: Boat
}

pub struct EchosounderParameters {
    pub max_limit: f64,
    pub min_limit: f64,
    pub pulse_repetition_interval: usize,
    pub pulse_length: usize,
    pub uses_high_frecuency: bool,
    pub angle: f32,
    pub transmited_potency: f64,
    pub gain: f32,
    pub echosounder_velocity: usize,
}

pub enum Boat {
    Small { speed: f64, balance_index: usize },
    Medium { speed: f64, balance_index: usize},
    Large { speed: f64, balance_index: usize}
}