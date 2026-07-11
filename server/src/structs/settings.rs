use serde::{Deserialize, Serialize};
use simulations::structs::simulation_constants::{
    EchosounderConstants, EnvironmentConstants, SimulationConstants,
};
use std::collections::HashMap;

/// Tipos de valores que puede tener una entrada del archivo de configuración.
pub enum ConfigValue {
    String(String),
    Int(i32),
    Float(f64),
}

// Struct de todas las configuraciones que se deben cargar del "config.toml"
// Esta como una structura fija para
// Evitar hacer matchs en cada uso, por si no existen
// Evitar levantar el servidor, si faltan valores necesarios.
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub port: i32,
    pub cache_amount: usize,
    pub db_name: String,
    pub storage_path: String,
    pub log_file_name: String,
    pub logging_type: i32,
    pub simplified_terminal_logs: bool,
    #[serde(skip)]
    pub admin_pass: String,
    pub maptiler_api_key: String,
    pub azimut_min: f64,
    pub azimut_max: f64,
    pub separation_min: f64,
    pub transport_speed_min: f64,
    pub transport_speed_max: f64,
    pub echo_depth_min: f64,
    pub echo_depth_max: f64,
    pub echo_pulse_min: f64,
    pub echo_pulse_max: f64,
    pub echo_umbral_min: f64,
    pub echo_umbral_max: f64,
    pub sound_speed_min: f64,
    pub sound_speed_max: f64,
    pub echo_diameter: f64,
    pub echo_high_freq_hz: f64,
    pub echo_high_freq_alpha: f64,
    pub echo_low_freq_hz: f64,
    pub echo_low_freq_alpha: f64,
    pub echo_beam_width_factor: f64,
    pub echo_multihaz_angle_deg: f64,
    pub echo_detection_threshold: f64,
    pub echo_max_gain: f64,
    pub sound_velocity: f64,
    pub tide_amplitude: f64,
    pub tide_period_h: f64,
    pub tide_phase: f64,
}

impl Settings {
    /// Arma el struct de constantes fisicas que necesita `simulations`, a
    /// partir de los valores ya cargados de config.toml. `simulations` no
    /// sabe nada de `Settings` ni de config.toml — solo recibe estos datos.
    pub fn simulation_constants(&self) -> SimulationConstants {
        SimulationConstants {
            echosounder: EchosounderConstants {
                diameter: self.echo_diameter,
                high_freq_hz: self.echo_high_freq_hz,
                high_freq_alpha: self.echo_high_freq_alpha,
                low_freq_hz: self.echo_low_freq_hz,
                low_freq_alpha: self.echo_low_freq_alpha,
                beam_width_factor: self.echo_beam_width_factor,
                multihaz_angle_deg: self.echo_multihaz_angle_deg,
                detection_threshold: self.echo_detection_threshold,
                max_gain: self.echo_max_gain,
            },
            environment: EnvironmentConstants {
                sound_velocity: self.sound_velocity,
                tide_amplitude: self.tide_amplitude,
                tide_period_h: self.tide_period_h,
                tide_phase: self.tide_phase,
            },
        }
    }
}

/// Construye los Settings a partir del HashMap leído del config, validando cada clave.
/// Si falta alguna obligatoria o tiene el tipo equivocado, devuelve Err y la app no arranca
/// (salvo `FILE_STORAGE_PATH`, que cae a "./storage" por defecto).
impl TryFrom<HashMap<String, ConfigValue>> for Settings {
    type Error = String;

    fn try_from(config: HashMap<String, ConfigValue>) -> Result<Self, Self::Error> {
        // Aca se agrega y relaciona con el archivo.
        Ok(Settings {
            port: get_int(&config, "PORT")?,
            cache_amount: get_usize(&config, "CACHE_ITEMS_MAX")?,
            db_name: get_string(&config, "DB_NAME")?,
            admin_pass: get_string(&config, "ADMIN_PASS")?,
            maptiler_api_key: get_string(&config, "MAPTILER_API_KEY")?,
            azimut_min: get_float(&config, "AZIMUT_MIN")?,
            azimut_max: get_float(&config, "AZIMUT_MAX")?,
            separation_min: get_float(&config, "SEPARATION_MIN")?,
            transport_speed_min: get_float(&config, "TRANSPORT_SPEED_MIN")?,
            transport_speed_max: get_float(&config, "TRANSPORT_SPEED_MAX")?,
            echo_depth_min: get_float(&config, "ECHO_DEPTH_MIN")?,
            echo_depth_max: get_float(&config, "ECHO_DEPTH_MAX")?,
            echo_pulse_min: get_float(&config, "ECHO_PULSE_MIN")?,
            echo_pulse_max: get_float(&config, "ECHO_PULSE_MAX")?,
            echo_umbral_min: get_float(&config, "ECHO_UMBRAL_MIN")?,
            echo_umbral_max: get_float(&config, "ECHO_UMBRAL_MAX")?,
            sound_speed_min: get_float(&config, "SOUND_SPEED_MIN")?,
            sound_speed_max: get_float(&config, "SOUND_SPEED_MAX")?,
            echo_diameter: get_float(&config, "ECHO_DIAMETER")?,
            echo_high_freq_hz: get_float(&config, "ECHO_HIGH_FREQ_HZ")?,
            echo_high_freq_alpha: get_float(&config, "ECHO_HIGH_FREQ_ALPHA")?,
            echo_low_freq_hz: get_float(&config, "ECHO_LOW_FREQ_HZ")?,
            echo_low_freq_alpha: get_float(&config, "ECHO_LOW_FREQ_ALPHA")?,
            sound_velocity: get_float(&config, "SOUND_VELOCITY")?,
            echo_beam_width_factor: get_float(&config, "ECHO_BEAM_WIDTH_FACTOR")?,
            echo_multihaz_angle_deg: get_float(&config, "ECHO_MULTIHAZ_ANGLE_DEG")?,
            echo_detection_threshold: get_float(&config, "ECHO_DETECTION_THRESHOLD")?,
            echo_max_gain: get_float(&config, "ECHO_MAX_GAIN")?,
            tide_amplitude: get_float(&config, "TIDE_AMPLITUDE")?,
            tide_period_h: get_float(&config, "TIDE_PERIOD_H")?,
            tide_phase: get_float(&config, "TIDE_PHASE")?,
            log_file_name: get_string(&config, "LOG_FILE_NAME")?,
            logging_type: get_int(&config, "LOGGING_TYPE")?,
            simplified_terminal_logs: int_to_bool(get_int(&config, "SIMPLIFIED_TERMINAL_LOGS")?),
            storage_path: get_string(&config, "FILE_STORAGE_PATH")
                .unwrap_or("./storage".to_string()),
        })
    }
}

// get_usize / get_int / get_string / get_float: leen una clave del config y validan su tipo.
// Devuelven Err con un mensaje claro si la clave falta o no es del tipo esperado.
fn get_usize(config: &HashMap<String, ConfigValue>, key: &str) -> Result<usize, String> {
    match config.get(key) {
        Some(ConfigValue::Int(v)) => Ok(*v as usize),
        Some(_) => Err(format!("'{key}' no es un entero")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_int(config: &HashMap<String, ConfigValue>, key: &str) -> Result<i32, String> {
    match config.get(key) {
        Some(ConfigValue::Int(v)) => Ok(*v),
        Some(_) => Err(format!("'{key}' no es un entero")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_string(config: &HashMap<String, ConfigValue>, key: &str) -> Result<String, String> {
    match config.get(key) {
        Some(ConfigValue::String(v)) => Ok(v.clone()),
        Some(_) => Err(format!("'{key}' no es un string")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_float(config: &HashMap<String, ConfigValue>, key: &str) -> Result<f64, String> {
    match config.get(key) {
        Some(ConfigValue::Float(v)) => Ok(*v),
        Some(_) => Err(format!("'{key}' no es un float")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn int_to_bool(value: i32) -> bool {
    value == 1
}
