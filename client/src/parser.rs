use common::{EchosounderParameters, PathParameters, GnssType, TransportParameters};
use crate::structs::state::{PathState, EchoState};
use crate::structs::limits::ConfigLimits;

pub fn parse_path_parameters(state: &PathState, limits: &ConfigLimits) -> Result<PathParameters, String> {
    let separacion = state.separacion.parse::<f64>()
        .map_err(|_| "Error: La separación debe ser un número válido".to_string())?;
    if separacion < limits.separation_min {
        return Err(format!("Error: La separación debe ser mayor a {} metro(s)", limits.separation_min));
    }
    
    let azimut = state.azimut.parse::<f64>()
        .map_err(|_| "Error: El azimut debe ser un número válido".to_string())?;
    if azimut < limits.azimut_min || azimut > limits.azimut_max {
        return Err(format!("Error: El azimut debe estar entre {} y {} grados", limits.azimut_min, limits.azimut_max));
    }

    let gnss_type = match state.gnss_type.as_str() {
        "Corrección de Fase" => GnssType::PhaseCorrection,
        "Corrección DGPS" => GnssType::DGPSCorrection,
        _ => GnssType::NoCorrection,
    };

    Ok(PathParameters { separacion, azimut, gnss_type })
}

pub fn parse_echosounder_parameters(state: &EchoState, limits: &ConfigLimits) -> Result<EchosounderParameters, String> {
    let max_limit = state.max_limit.parse::<f64>()
        .map_err(|_| "Error: Límite máximo inválido".to_string())?;
    if max_limit < limits.echo_depth_min || max_limit > limits.echo_depth_max {
        return Err(format!("Error: La profundidad maxima de medición de la ecosonda debe estar entre {} y {} metros", limits.echo_depth_min, limits.echo_depth_max));
    }
    let min_limit = state.min_limit.parse::<f64>()
        .map_err(|_| "Error: Límite mínimo inválido".to_string())?;
    if min_limit < limits.echo_depth_min || min_limit > limits.echo_depth_max {
        return Err(format!("Error: La profundidad minima de medicion de la ecosonda debe estar entre {} y {} metros", limits.echo_depth_min, limits.echo_depth_max));
    }
    if min_limit >= max_limit {
        return Err("Error: El límite mínimo debe ser menor al máximo".to_string());
    }
        
    let pulse_repetition_interval = state.pulse_repetition_interval.parse::<f64>()
        .map_err(|_| "Error: Intervalo de repetición inválido".to_string())?;
    if pulse_repetition_interval < limits.echo_pulse_min || pulse_repetition_interval > limits.echo_pulse_max {
        return Err(format!("Error: El intervalo de repetición debe estar entre {} y {} Hz", limits.echo_pulse_min, limits.echo_pulse_max));
    }
        
    let transmited_potency = state.transmited_potency.parse::<f64>()
        .map_err(|_| "Error: Potencia transmitida inválida".to_string())?;
        
    let gain = state.gain.parse::<f64>()
        .map_err(|_| "Error: Ganancia inválida".to_string())?;
        
        
    let threshold = state.umbral.parse::<f64>()
        .map_err(|_| "Error: Umbral inválido".to_string())?;
    if threshold < limits.echo_umbral_min || threshold > limits.echo_umbral_max {
        return Err(format!("Error: El umbral debe estar entre {}% y {}% ", limits.echo_umbral_min, limits.echo_umbral_max));
    }

    let sound_speed = state.sound_speed.parse::<f64>()
        .map_err(|_| "Error: Velocidad del sonido inválida".to_string())?;
    if sound_speed < limits.sound_speed_min || sound_speed > limits.sound_speed_max {
        return Err(format!("Error: La velocidad del sonido debe estar entre {} y {} m/s", limits.sound_speed_min, limits.sound_speed_max));
    }

    Ok(EchosounderParameters {
        mode: state.mode,
        angle: 0.0,
        absortion_coefficient: 0.0,
        max_limit,
        min_limit,
        pulse_repetition_interval,
        sound_speed,
        uses_high_frecuency: state.uses_high_frecuency,
        transmited_potency,
        gain,
        threshold
    })
}

pub fn parse_transport_parameters(state: &EchoState, limits: &ConfigLimits) -> Result<TransportParameters, String> {
    let speed = state.speed.parse::<f64>()
        .map_err(|_| "Error: La velocidad de la embarcación debe ser un número válido".to_string())?;
    if speed < limits.transport_speed_min || speed > limits.transport_speed_max {
        return Err(format!("Error: La velocidad de la embarcación debe estar entre {} y {} m/s", limits.transport_speed_min, limits.transport_speed_max));
    }

    Ok(TransportParameters {
        transport: state.transport,
        speed,
        uses_mareograph: state.uses_mareograph,
        uses_sound_profiler: state.uses_sound_profiler,
        uses_inertial_sensor: state.uses_inertial_sensor,
    })
}