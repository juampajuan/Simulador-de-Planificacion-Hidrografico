use common::{EchosounderParameters, PathParameters, TransportParameters, GnssType, EcosondaMode, Transport};

use crate::db::engine::DBEngine;

/// TODO: Mover a algo compartido??
pub struct StudentSimulation {
    pub id: i64,
    pub selected: bool,

    pub result_min_depth: f64,
    pub result_max_depth: f64,

    pub student_id: i64,
    pub project_id: i64,

    pub path_parameters: PathParameters,
    pub transport_parameters: TransportParameters,
    pub echosounder_parameters: EchosounderParameters,
}


/// Agregar una entrada de student_simualions a la DB
/// La misma almacene los parametros de un intento de simulacion a la DB
/// Usado para las correciones del docente.
pub fn create_student_simulation(
    db: &DBEngine,
    student_id: i64,
    project_id: i64,
    result_min_depth: f64,
    result_max_depth: f64,
    path: &PathParameters,
    transport: &TransportParameters,
    echo: &EchosounderParameters,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO student_simulations(
            result_max_depth,
            result_min_depth,

            separation,
            azimuth,
            gnss,

            transport,
            transport_speed,
            uses_mareograph,
            uses_sound_profiler,
            uses_inertial_sensor,

            echosounder_mode,
            uses_high_frequency,

            min_depth,
            max_depth,
            pulse_repetition_interval,
            sound_speed,
            transmitted_potency,
            threshold,
            gain,

            student_id,
            project_id
        )
        VALUES(
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?, ?, ?,
            ?, ?,
            ?, ?, ?, ?, ?, ?, ?,
            ?, ?
        )
        "
    )?;

    statement.bind((1, 0))?;
    statement.bind((1, result_max_depth))?;
    statement.bind((2, result_min_depth))?;

    statement.bind((3, path.separacion))?;
    statement.bind((4, path.azimut))?;
    statement.bind((5, path.gnss_type as i64))?;

    statement.bind((6, transport.transport as i64))?;
    statement.bind((7, transport.speed))?;
    statement.bind((8, transport.uses_mareograph as i64))?;
    statement.bind((9, transport.uses_sound_profiler as i64))?;
    statement.bind((10, transport.uses_inertial_sensor as i64))?;

    statement.bind((11, echo.mode as i64))?;
    statement.bind((12, echo.uses_high_frecuency as i64))?;

    statement.bind((13, echo.min_limit))?;
    statement.bind((14, echo.max_limit))?;
    statement.bind((15, echo.pulse_repetition_interval))?;
    statement.bind((16, echo.sound_speed))?;
    statement.bind((17, echo.transmited_potency))?;
    statement.bind((18, echo.threshold))?;
    statement.bind((19, echo.gain))?;

    statement.bind((20, student_id))?;
    statement.bind((21, project_id))?;

    statement.next()?;

    Ok(())
}

/// Permite marcar 1 simulacion como la entrega.
/// TODO: Pregunntar a fernando, si deberia de poder cambiarlo? Si toco mal.
pub fn select_student_simulation(
    db: &DBEngine,
    simulation_id: i64,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        UPDATE student_simulations
        SET selected = TRUE
        WHERE id = ?
        "
    )?;

    statement.bind((1, simulation_id))?;
    statement.next()?;

    Ok(())
}

/// Permite obtener todos los parametros de los intentos de simulacion
/// Usado tanto por docente como por alumnos
pub fn get_student_simulations(
    db: &DBEngine,
    student_id: i64,
) -> Result<Vec<StudentSimulation>, sqlite::Error> {

    let mut statement = db.run_query(
        "
        SELECT *
        FROM student_simulations
        WHERE student_id = ?
        ORDER BY id
        "
    )?;

    statement.bind((1, student_id))?;

    let mut simulations = Vec::new();

    while let sqlite::State::Row = statement.next()? {

        simulations.push(StudentSimulation {
            id: statement.read("id")?,
            selected: statement.read::<i64, _>("selected")? == 1,

            result_max_depth: statement.read("result_max_depth")?,
            result_min_depth: statement.read("result_min_depth")?,

            student_id: statement.read("student_id")?,
            project_id: statement.read("project_id")?,

            path_parameters: PathParameters {
                separacion: statement.read("separation")?,
                azimut: statement.read("azimuth")?,
                gnss_type: GnssType::try_from(statement.read::<i64, _>("gnss")?)
                .map_err(|_| sqlite::Error {
                    code: None,
                    message: Some("Invalid GnssType".to_string()),
                })?,
            },

            transport_parameters: TransportParameters {
                transport: Transport::try_from(statement.read::<i64, _>("transport")?)
                .map_err(|_| sqlite::Error {
                    code: None,
                    message: Some("Invalid Transport".to_string()),
                })?,
                speed: statement.read("transport_speed")?,
                uses_mareograph: statement.read::<i64, _>("uses_mareograph")? == 1,
                uses_sound_profiler: statement.read::<i64, _>("uses_sound_profiler")? == 1,
                uses_inertial_sensor: statement.read::<i64, _>("uses_inertial_sensor")? == 1,
            },

            echosounder_parameters: EchosounderParameters {
                mode: EcosondaMode::try_from(statement.read::<i64, _>("echosounder_mode")?)
                .map_err(|_| sqlite::Error {
                    code: None,
                    message: Some("Invalid EcosondaMode".to_string()),
                })?,
                angle: 0.0,
                absortion_coefficient: 0.0,
                max_limit: statement.read("max_depth")?,
                min_limit: statement.read("min_depth")?,
                pulse_repetition_interval: statement.read("pulse_repetition_interval")?,
                uses_high_frecuency: statement.read::<i64, _>("uses_high_frequency")? == 1,
                transmited_potency: statement.read("transmitted_potency")?,
                gain: statement.read("gain")?,
                threshold: statement.read("threshold")?,
                sound_speed: statement.read("sound_speed")?,
            },
        });
    }

    Ok(simulations)
}