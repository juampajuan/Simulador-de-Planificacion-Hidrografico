use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// pub enum Types {
//     String,
//     I32,
// }

// // TODO: Implementar, para comunicar los parametros en los POST entre client-server
// pub fn parse_string(line: String) -> Option<HashMap<String, Types>> {
//     let mut map = HashMap::new();
 
//     // Procesamos la linea y las dejamos en el HashMap
//     // key1=numero;key2="string"; .....

//     Some(map)
// }

// pub fn serialize_map(map: HashMap<String, Types>) -> Option<String> {
//     // Procesamos el HashMap y lo transformamos en la linea.
//     // key1=numero;key2="string"; .....

//     Some("".to_string())
// }


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Boat {
    W,
    Y,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EcosondaMode {
    Monohaz {
        angle:f64,
        absortion_coefficient: f64,
    },       
    Multihaz,       
}

// Structs principales

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PathParameters {
    pub separacion: f64,
    pub azimut: f64,
    pub gnss_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EchosounderParameters {
    pub uses_monohaz: bool,
    pub mode: Option<EcosondaMode>,
    pub max_limit: f64,
    pub min_limit: f64,
    pub pulse_repetition_interval: f64, // ms
    pub pulse_length: usize,
    pub uses_high_frecuency: bool,
    pub transmited_potency: f64,
    pub gain: f32,
    pub echosounder_velocity: usize,
    pub threshold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StudentMeasuringParameters {
    pub uses_mathegapher: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
    pub echo_sounder_parameters: EchosounderParameters,
    pub boat: Boat,
}