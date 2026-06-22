use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::structs::settings::{Settings, ConfigValue};

/// Carga la configuración desde `config.toml`: abre el archivo, lo parsea y construye
/// los `Settings`. Devuelve Err si no se puede abrir o si la config es inválida.
pub fn load_settings() -> Result<Settings, String> {
    let file = match File::open("config.toml") {
        Ok(file) => file,
        Err(_) => return Err("No se pudo abrir config.toml".to_string()),
    };

    let config = file_parser(file)?;
    Settings::try_from(config)
}

/// Lee el archivo línea por línea y arma el mapa de claves/valores de configuración.
fn file_parser(file: File) -> Result<HashMap<String, ConfigValue>, String> {

    let mut config: HashMap<String, ConfigValue> = HashMap::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return Err("Error leyendo una línea".to_string()),
        };

        parse_line(line, &mut config)?;
    }

    Ok(config)
}

/// Parsea una línea de config con formato `clave = valor`. Ignora líneas vacías y comentarios
/// (`#`), recorta comentarios al final de línea, e infiere el tipo del valor: string, int o float.
/// Devuelve Err si la línea no tiene formato válido.
fn parse_line(line:String, config: &mut HashMap<String, ConfigValue>) -> Result<(), String> {

    let line = line.trim();

    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }

    let line = match line.split_once('#') {
        Some((code, _comment)) => code.trim(),
        None => line,
    };

    let (key, value) = match line.split_once('=') {
        Some(parts) => parts,
        None =>  return Err("Archivo corrupto".to_string())
    };

    let key = key.trim().to_string();
    let value = value.trim();

    if value.starts_with('"') && value.ends_with('"') {

        let value = value
            .trim_matches('"')
            .to_string();

        config.insert(
            key,
            ConfigValue::String(value)
        );

        return Ok(());
    }
 
    if let Ok(number) = value.parse::<i32>() {

        config.insert(
            key,
            ConfigValue::Int(number)
        );

        return Ok(());
    }

    if let Ok(number) = value.parse::<f64>() {

        config.insert(
            key,
            ConfigValue::Float(number)
        );

        return Ok(());
    }

    Err("Archivo Corrupto".to_string())
}