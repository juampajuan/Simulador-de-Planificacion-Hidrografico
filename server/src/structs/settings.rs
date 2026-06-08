use std::collections::HashMap;

// Los tipos de datos que se aceptan en el config
pub enum ConfigValue {
    String((String)),
    Int(i32), 
}

// Las distintas configs.
// Se deben agregar a mano.
// Con esto evitamos un match cada vez que necesitas leerlas.
// Si no lo puede generar, no arranca la aplicacion
pub struct Settings {
    pub port: i32,
    pub cache_amount: usize,
    pub db_name: String
}


impl TryFrom<HashMap<String, ConfigValue>> for Settings {
    type Error = String;

    fn try_from(config: HashMap<String, ConfigValue>) -> Result<Self, Self::Error> {
        // Aca se agrega y relaciona con el archivo.
        Ok(Settings {
            port: get_int(&config, "PORT")?,
            cache_amount: get_usize(&config, "CACHE_ITEMS_MAX")?,
            db_name: get_string(&config, "DB_NAME")?,
        })
    }
}

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
