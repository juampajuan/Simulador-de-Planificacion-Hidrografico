use std::collections::HashMap;

pub enum Types {
    String,
    I32,
}

// TODO: Implementar, para comunicar los parametros en los POST entre client-server
pub fn parse_string(line: String) -> Option<HashMap<String, Types>> {
    let mut map = HashMap::new();
 
    // Procesamos la linea y las dejamos en el HashMap
    // key1=numero;key2="string"; .....

    Some(map)
}

pub fn serialize_map(map: HashMap<String, Types>) -> Option<String> {
    // Procesamos el HashMap y lo transformamos en la linea.
    // key1=numero;key2="string"; .....

    Some("".to_string())
}