use common::{EchosounderParameters, PathParameters, GnssType, TransportParameters};
use crate::requests::{PathState, EchoState};

pub fn parse_path_parameters(state: &PathState) -> Result<PathParameters, String> {
    let separacion = state.separacion.parse::<f64>()
        .map_err(|_| "Error: La separación debe ser un número válido".to_string())?;
        
    let azimut = state.azimut.parse::<f64>()
        .map_err(|_| "Error: El azimut debe ser un número válido".to_string())?;

    let gnss_type = match state.gnss_type.as_str() {
        "Fase" => GnssType::PhaseCorrection,
        "DGPS" => GnssType::DGPSCorrection,
        _ => GnssType::NoCorrection,
    };

    Ok(PathParameters { separacion, azimut, gnss_type })
}

pub fn parse_echosounder_parameters(state: &EchoState) -> Result<EchosounderParameters, String> {
    let max_limit = state.max_limit.parse::<f64>()
        .map_err(|_| "Error: Límite máximo inválido".to_string())?;
        
    let min_limit = state.min_limit.parse::<f64>()
        .map_err(|_| "Error: Límite mínimo inválido".to_string())?;
        
    let pulse_repetition_interval = state.pulse_repetition_interval.parse::<f64>()
        .map_err(|_| "Error: Intervalo de repetición inválido".to_string())?;
        
    let pulse_length = state.pulse_length.parse::<usize>()
        .map_err(|_| "Error: Largo de pulso inválido".to_string())?;
        
    let transmited_potency = state.transmited_potency.parse::<f64>()
        .map_err(|_| "Error: Potencia transmitida inválida".to_string())?;
        
    let gain = state.gain.parse::<f32>()
        .map_err(|_| "Error: Ganancia inválida".to_string())?;
        
    let echosounder_velocity = state.echosounder_velocity.parse::<usize>()
        .map_err(|_| "Error: Velocidad de ecosonda inválida".to_string())?;
        
    let threshold = state.umbral.parse::<f64>()
        .map_err(|_| "Error: Umbral inválido".to_string())?;

    Ok(EchosounderParameters {
        mode: state.mode,
        angle: 0.0,
        absortion_coefficient: 0.0,
        max_limit,
        min_limit,
        pulse_repetition_interval,
        pulse_length,
        uses_high_frecuency: state.uses_high_frecuency,
        transmited_potency,
        gain,
        echosounder_velocity,
        threshold,
    })
}

pub fn parse_transport_parameters(state: &EchoState) -> Result<TransportParameters, String> {
    let speed = state.speed.parse::<f64>()
        .map_err(|_| "Error: La velocidad de la embarcación debe ser un número válido".to_string())?;

    Ok(TransportParameters {
        transport: state.transport,
        speed,
        uses_mareograph: state.uses_mareograph,
        uses_sound_profiler: state.uses_sound_profiler,
        uses_inertial_sensor: state.uses_inertial_sensor,
    })
}