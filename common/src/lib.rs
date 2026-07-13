use serde::{Deserialize, Serialize};

// Tipos de datos compartidos entre server, client y simulations.
// Centraliza acá los parámetros y respuestas que viajan entre procesos para no
// duplicar las mismas estructuras en cada crate.

/// Tipo de corrección GNSS aplicada al posicionamiento del recorrido:
/// sin corrección, DGPS (submétrica) o por fase (la más precisa).
#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum GnssType {
    #[default]
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum Transport {
    #[default]
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum EcosondaMode {
    #[default]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PathParameters {
    pub separacion: f64,
    pub azimut: f64,
    pub gnss_type: GnssType,
}

/// Configuración completa de la ecosonda: modo, apertura, rango de profundidad,
/// intervalo de pulso, frecuencia, potencia, ganancia, umbral de detección y
/// velocidad del sonido. Son las perillas que ajusta el alumno antes de medir.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub struct TransportParameters {
    pub transport: Transport,
    pub speed: f64, // m/s
    pub uses_mareograph: bool,
    pub uses_sound_profiler: bool,
    pub uses_inertial_sensor: bool,
}

/// Todo lo que el alumno configura para una medición: los parámetros de ecosonda
/// y los de la embarcación juntos.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub struct StudentMeasuringParameters {
    pub echo_sounder_parameters: EchosounderParameters,
    pub transport_parameters: TransportParameters,
}

/// Resultado de una simulación listo para mostrar en el front.
/// Almacena los metadatos de las profundidades calculadas y las rutas
/// relativas a los archivos de imágenes físicas guardados en el servidor.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SimulationResponse {
    pub real_min_depth: f64,
    pub real_max_depth: f64,
    pub interpolation_min_depth: f64,
    pub interpolation_max_depth: f64,
    pub simulation_image_path: Option<String>,
    pub coverage_image_path: Option<String>,
    pub difference_image_path: Option<String>,
}

/// Un intento de simulación guardado. Usado tanto por el server (para dar de alta
/// una fila nueva) como por el client (para deserializar el historial que le llega
/// del endpoint)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StudentSimulationData {
    pub attempt_number: i64,

    pub result_min_depth: f64,
    pub result_max_depth: f64,

    pub student_id: i64,
    pub project_id: i64,

    pub path_parameters: PathParameters,
    pub transport_parameters: TransportParameters,
    pub echosounder_parameters: EchosounderParameters,

    pub simulation_image_path: Option<String>,
    pub coverage_image_path: Option<String>,
    pub difference_image_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StudentSimulation {
    pub id: i64,
    pub selected: bool,
    #[serde(flatten)]
    pub data: StudentSimulationData,
}

/// Toda la configuración de un proyecto que carga el profesor.
/// Compartido entre server y client
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: Option<String>,
    pub attempts_limit: i64,
    pub exam_mode: bool,
    pub due_date: Option<String>,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: f64,
    pub geotiff_min_depth: f64,
    pub geotiff_max_depth: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AdminProjectView {
    pub id: usize,
    pub filename: String,
    pub professor_id: i64,
    #[serde(flatten)]
    pub metadata: ProjectMetadata,
}

/// Coordenadas (lat, lon) de las cuatro esquinas del geotiff y su centro,
/// usadas por el cliente para centrar y ajustar el zoom del mapa de fondo.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GeoCorners {
    pub sup_izq: (f64, f64),
    pub sup_der: (f64, f64),
    pub inf_izq: (f64, f64),
    pub inf_der: (f64, f64),
    pub centro: (f64, f64),
}

/// Respuesta del endpoint `/student_project`: los datos del proyecto (aplanados en el JSON),
/// los intentos ya gastados por el alumno, las coordenadas geográficas del geotiff y la
/// API key de MapTiler para dibujar el mapa de fondo.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StudentProjectResponse {
    #[serde(flatten)]
    pub project: AdminProjectView,
    pub attempts_spent: i64,
    pub coordinates: GeoCorners,
    pub maptiler_api_key: String,
}

/// Datos para dar de alta un alumno nuevo. El server lo deserializa del body
/// del request, el client lo serializa para mandarlo
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NewStudent {
    pub name: String,
    pub project_id: i64,
}

/// Body que manda el alumno al pedir una simulación: los parámetros de ecosonda
/// y los del recorrido. `echo_parameters` es `Option` porque el endpoint `/create_path`
/// reusa este mismo struct para parsear su body, que no incluye esos datos
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FullSimulationRequest {
    #[serde(default)]
    pub echo_parameters: Option<StudentMeasuringParameters>,
    pub path_parameters: PathParameters,
}

/// Un alumno dado de alta por un profesor.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Student {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub project_id: i64,
    pub attempts: i64,
}
