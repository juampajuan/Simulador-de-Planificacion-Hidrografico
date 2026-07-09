use serde::{Deserialize, Serialize};

// Tipos de datos compartidos entre server, client y simulations.
// Centraliza acá los parámetros y respuestas que viajan entre procesos para no
// duplicar las mismas estructuras en cada crate.


/// Tipo de corrección GNSS aplicada al posicionamiento del recorrido:
/// sin corrección, DGPS (submétrica) o por fase (la más precisa).
#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GnssType {
    NoCorrection = 0,
    DGPSCorrection = 1,
    PhaseCorrection = 2,
}

// Metodo para obtenerlo del INTEGER de la DB.
impl TryFrom<i64> for GnssType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GnssType::NoCorrection),
            1 => Ok(GnssType::DGPSCorrection),
            2 => Ok(GnssType::PhaseCorrection),
            _ => Err(()),
        }
    }
}

/// Tipo de embarcación. Influye en cuánto se sacude con las olas:
/// un barco (Ship) se mueve menos que un bote (Boat) o una lancha (Launch)
#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Transport {
    Ship = 0,
    Boat = 1,
    Launch = 2,
}

// Metodo para obtenerlo del INTEGER de la DB.
impl TryFrom<i64> for Transport {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Transport::Ship),
            1 => Ok(Transport::Boat),
            2 => Ok(Transport::Launch),
            _ => Err(()),
        }
    }
}


/// Modo de la ecosonda: monohaz mide un punto por pulso, multihaz mide una franja.
#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EcosondaMode {
    Monohaz = 0,
    Multihaz = 1,      
}

// Metodo para obtenerlo del INTEGER de la DB.
impl TryFrom<i64> for EcosondaMode {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EcosondaMode::Monohaz),
            1 => Ok(EcosondaMode::Multihaz), 
            _ => Err(()),
        }
    }
}

/// Parámetros del recorrido del barco: separación entre líneas, azimut (orientación)
/// y tipo de corrección GNSS.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PathParameters {
    pub separacion: f64,
    pub azimut: f64,
    pub gnss_type: GnssType,
}

/// Configuración completa de la ecosonda: modo, apertura, rango de profundidad,
/// intervalo de pulso, frecuencia, potencia, ganancia, umbral de detección y
/// velocidad del sonido. Son las perillas que ajusta el alumno antes de medir.
#[derive(Debug, Serialize, Deserialize, Clone,Copy, PartialEq)]
pub struct EchosounderParameters {
    pub mode: EcosondaMode,
    pub angle: f64,
    pub absortion_coefficient: f64,
    pub max_limit: f64,
    pub min_limit: f64,
    pub pulse_repetition_interval: f64, // Hz, en simulation lo convertimos a segundos.
    pub uses_high_frecuency: bool,
    pub transmited_potency: f64,
    pub gain: f64,
    pub threshold: f64,
    pub sound_speed: f64,
}

/// Configuración de la embarcación: tipo, velocidad y qué instrumentos de corrección
/// lleva activos (mareógrafo, perfilador de sonido, sensor inercial).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct TransportParameters {
    pub transport: Transport,
    pub speed: f64, // m/s
    pub uses_mareograph: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
}

/// Todo lo que el alumno configura para una medición: los parámetros de ecosonda
/// y los de la embarcación juntos.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct StudentMeasuringParameters {
    pub echo_sounder_parameters: EchosounderParameters,
    pub transport_parameters: TransportParameters,
}

/// Resultado de una simulación listo para mostrar en el front. Las dos imágenes (el mapa
/// interpolado y la barra de escala) viajan como texto base64 
/// `min_depth`/`max_depth` se usan para rotular la escala de colores.
#[derive(serde::Serialize, serde::Deserialize, Clone)]

pub struct SimulationBase64Response {
    pub min_depth: f64,
    pub max_depth: f64,
    pub map_base64: String,
    pub scale_base64: String,
    pub parameters: StudentMeasuringParameters,
}