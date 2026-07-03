/// Todas las constantes fisicas de la simulacion
/// Se separa en dos grupos segun a que corresponde cada constante:
/// `echosounder` (propiedades del instrumento) y `environment` (propiedades
/// del ambiente/agua, como la marea, que no tienen que ver con la sonda).


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationConstants {
    pub echosounder: EchosounderConstants,
    pub environment: EnvironmentConstants,
}

/// Constantes fisicas propias de la ecosonda (el instrumento).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EchosounderConstants {
    /// Diametro del transductor, en metros.
    pub diameter: f64,
    /// Frecuencia en modo alta frecuencia, en Hz.
    pub high_freq_hz: f64,
    /// Coeficiente de absorcion en alta frecuencia, en dB/m.
    pub high_freq_alpha: f64,
    /// Frecuencia en modo baja frecuencia, en Hz.
    pub low_freq_hz: f64,
    /// Coeficiente de absorcion en baja frecuencia, en dB/m.
    pub low_freq_alpha: f64,
    /// Factor para el calculo del ancho del haz Monohaz (Clase 03b).
    pub beam_width_factor: f64,
    /// Angulo TOTAL del haz Multihaz, en grados (no el semi-angulo).
    pub multihaz_angle_deg: f64,
    /// Umbral minimo de senal para que la sonda detecte el eco, en dB.
    pub detection_threshold: f64,
    /// Ganancia maxima que puede configurar el alumno, en dB. A esa
    /// ganancia se simula un eco falso (profundidad 90% de la real).
    pub max_gain: f64,
}

/// Constantes del ambiente/agua, sin relacion con el instrumento.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvironmentConstants {
    /// Velocidad del sonido en el agua, en m/s.
    pub sound_velocity: f64,
    /// Amplitud de la marea, en metros.
    pub tide_amplitude: f64,
    /// Periodo de la marea, en horas.
    pub tide_period_h: f64,
    /// Fase inicial de la marea, en radianes.
    pub tide_phase: f64,
}