use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::structs::settings::{Settings, ConfigValue};


pub fn load_settings() -> Result<Settings, String> {
    let file = match File::open("config.toml") {
        Ok(file) => file,
        Err(_) => return Err(format!(
            "No se pudo abrir config.toml",
        ))  
    };

    let config = file_parser(file)?;
    Settings::try_from(config)
}

fn file_parser(file: File) -> Result<HashMap<String, ConfigValue>, String> {

    let mut config: HashMap<String, ConfigValue> = HashMap::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return Err(format!(
                "Error leyendo una línea"
            ))
        };

        if let Err(err) = parse_line(line, &mut config) {
            return Err(err);
        }

    }

    Ok(config)
}

fn parse_line(line:String, config: &mut HashMap<String, ConfigValue>) -> Result<(), String> {

    let line = line.trim();

    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }

    let (key, value) = match line.split_once('=') {
        Some(parts) => parts,
        None =>  return Err(format!("Archivo corrupto"))
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

    Err(format!("Archivo Corrupto"))
}