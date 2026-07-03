use common::EchosounderParameters;
use crate::structs::simulation_constants::SimulationConstants;

// ------------------------------------------------------------
//  Trait público
// ------------------------------------------------------------

pub trait EchosounderLogic {
    fn create_echosounder(&mut self, constants: &SimulationConstants);
}

// ------------------------------------------------------------
//  Implementación del trait
// ------------------------------------------------------------

impl EchosounderLogic for EchosounderParameters {
    fn create_echosounder(&mut self, constants: &SimulationConstants) {
        let (angulo_rad, alfa) = calculate_angle_and_absortion_coefficient(self.uses_high_frecuency, constants);
        self.angle = angulo_rad;
        self.absortion_coefficient = alfa;
    }
}

// ------------------------------------------------------------
//  Cálculo de parámetros físicos de la ecosonda
// ------------------------------------------------------------

fn calculate_angle_and_absortion_coefficient(uses_high_frecuency: bool, constants: &SimulationConstants) -> (f64, f64) {

    let (frecuencia, alfa) = if uses_high_frecuency {
        (constants.echosounder.high_freq_hz, constants.echosounder.high_freq_alpha)
    } else {
        (constants.echosounder.low_freq_hz, constants.echosounder.low_freq_alpha)
    };
    // sound_velocity es del agua (environment), no de la ecosonda.
    let angulo_grados: f64 = constants.echosounder.beam_width_factor * (constants.environment.sound_velocity / frecuencia) / constants.echosounder.diameter;
    (angulo_grados, alfa)
}