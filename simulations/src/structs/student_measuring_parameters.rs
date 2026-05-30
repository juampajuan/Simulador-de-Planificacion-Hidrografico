use common::{EchosounderParameters};

// ------------------------------------------------------------
//  Constantes físicas — Alta frecuencia (200 kHz, D=10cm)
// ------------------------------------------------------------

const HIGH_FREQ_HZ: f64        = 200000.0; // Frecuencia en Hz
const HIGH_FREQ_DIAMETER: f64  = 0.10;      // Diámetro del transductor en metros
const HIGH_FREQ_ALPHA: f64     = 0.060;     // Coeficiente de absorción en dB/m

// ------------------------------------------------------------
//  Constantes físicas — Baja frecuencia (12 kHz, D=20cm)
// ------------------------------------------------------------

const LOW_FREQ_HZ: f64         = 12000.0;  // Frecuencia en Hz
const LOW_FREQ_DIAMETER: f64   = 0.20;      // Diámetro del transductor en metros
const LOW_FREQ_ALPHA: f64      = 0.004;     // Coeficiente de absorción en dB/m

// ------------------------------------------------------------
//  Constantes del modelo de error
// ------------------------------------------------------------

const SOUND_VELOCITY: f64      = 1500.0; // Velocidad del sonido en el agua en m/s
const BEAM_WIDTH_FACTOR: f64   = 60.0;   // Factor para cálculo de ancho del haz (Clase 03b)



// ------------------------------------------------------------
//  Trait público
// ------------------------------------------------------------

pub trait EchosounderLogic {
    fn create_echosounder(&mut self);
}

// ------------------------------------------------------------
//  Implementación del trait
// ------------------------------------------------------------

impl EchosounderLogic for EchosounderParameters {
    fn create_echosounder(&mut self) {
        let (angulo_rad, alfa) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency);
        self.angle = angulo_rad;
        self.absortion_coefficient = alfa;
    }
}

// ------------------------------------------------------------
//  Cálculo de parámetros físicos de la ecosonda
// ------------------------------------------------------------


fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool) -> (f64, f64) {
    let (frecuencia, diametro, alfa) = if uses_high_frecuency {
        (HIGH_FREQ_HZ, HIGH_FREQ_DIAMETER, HIGH_FREQ_ALPHA)
    } else {
        (LOW_FREQ_HZ, LOW_FREQ_DIAMETER, LOW_FREQ_ALPHA)
    };
    let angulo_grados: f64 = BEAM_WIDTH_FACTOR * (SOUND_VELOCITY / frecuencia) / diametro;
    (angulo_grados, alfa)
}

