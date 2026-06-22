use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    pub upload_path: String,
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
}

/// Construye los Settings a partir del HashMap leído del config, validando cada clave.
/// Si falta alguna obligatoria o tiene el tipo equivocado, devuelve Err y la app no arranca
/// (salvo `FILE_UPLOAD_PATH`, que cae a "./uploads" por defecto).
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
            upload_path: get_string(&config, "FILE_UPLOAD_PATH").unwrap_or("./uploads".to_string()),
        })
    }
}

// get_usize / get_int / get_string / get_float: leen una clave del config y validan su tipo.
// Devuelven Err con un mensaje claro si la clave falta o no es del tipo esperado.
fn get_usize(
    config: &HashMap<String, ConfigValue>,
    key: &str,
) -> Result<usize, String> {
    match config.get(key) {
        Some(ConfigValue::Int(v)) => Ok(*v as usize),
        Some(_) => Err(format!("'{key}' no es un entero")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_int(
    config: &HashMap<String, ConfigValue>,
    key: &str,
) -> Result<i32, String> {
    match config.get(key) {
        Some(ConfigValue::Int(v)) => Ok(*v),
        Some(_) => Err(format!("'{key}' no es un entero")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_string(
    config: &HashMap<String, ConfigValue>,
    key: &str,
) -> Result<String, String> {
    match config.get(key) {
        Some(ConfigValue::String(v)) => Ok(v.clone()),
        Some(_) => Err(format!("'{key}' no es un string")),
        None => Err(format!("Falta '{key}'")),
    }
}

fn get_float(
    config: &HashMap<String, ConfigValue>,
    key: &str,
) -> Result<f64, String> {
    match config.get(key) {
        Some(ConfigValue::Float(v)) => Ok(*v),
        Some(_) => Err(format!("'{key}' no es un float")),
        None => Err(format!("Falta '{key}'")),
    }
}