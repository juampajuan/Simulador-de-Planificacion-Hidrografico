use common::{EchosounderParameters, PathParameters, TransportParameters, GnssType, EcosondaMode, Transport};

use crate::db::engine::DBEngine;

/// TODO: Mover a algo compartido??
#[derive(serde::Serialize)]
pub struct StudentSimulation {
    pub id: i64,
    pub attempt_number: i64,
    pub selected: bool,

    pub result_min_depth: f64,
    pub result_max_depth: f64,

    pub student_id: i64,
    pub project_id: i64,

    pub path_parameters: PathParameters,
    pub transport_parameters: TransportParameters,
    pub echosounder_parameters: EchosounderParameters,

    pub simulation_image_path: Option<String>,
}


/// Agregar una entrada de student_simualions a la DB
/// La misma almacene los parametros de un intento de simulacion a la DB
/// Usado para las correciones del docente.
pub fn create_student_simulation(
    db: &DBEngine,
    student_id: i64,
    project_id: i64,
    attempt_number: i64,
    result_min_depth: f64,
    result_max_depth: f64,
    path: &PathParameters,
    transport: &TransportParameters,
    echo: &EchosounderParameters,
    simulation_image_path: Option<&str>,
) -> Result<(), sqlite::Error> {

    let mut statement = db.run_query(
        "
        INSERT INTO student_simulations(
            attempt_number,
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
            project_id,

            simulation_image_path
        )
        VALUES(
            ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?, ?, ?,
            ?, ?,
            ?, ?, ?, ?, ?, ?, ?,
            ?, ?,
            ?
        )
        "
    )?;

    // statement.bind((1, 0))?;
    statement.bind((1, attempt_number))?;
    statement.bind((2, result_max_depth))?;
    statement.bind((3, result_min_depth))?;

    statement.bind((4, path.separacion))?;
    statement.bind((5, path.azimut))?;
    statement.bind((6, path.gnss_type as i64))?;

    statement.bind((7, transport.transport as i64))?;
    statement.bind((8, transport.speed))?;
    statement.bind((9, transport.uses_mareograph as i64))?;
    statement.bind((10, transport.uses_sound_profiler as i64))?;
    statement.bind((11, transport.uses_inertial_sensor as i64))?;

    statement.bind((12, echo.mode as i64))?;
    statement.bind((13, echo.uses_high_frecuency as i64))?;

    statement.bind((14, echo.min_limit))?;
    statement.bind((15, echo.max_limit))?;
    statement.bind((16, echo.pulse_repetition_interval))?;
    statement.bind((17, echo.sound_speed))?;
    statement.bind((18, echo.transmited_potency))?;
    statement.bind((19, echo.threshold))?;
    statement.bind((20, echo.gain))?;

    statement.bind((21, student_id))?;
    statement.bind((22, project_id))?;

    match simulation_image_path {
        Some(p) => statement.bind((23, p))?,
        None => statement.bind((23, ""))?,
    };

    statement.next()?;

    Ok(())
}

/// Remueve todas las entregas finales para un alumno específico, dejando todo en FALSE.
/// Se usa cuando el alumno desmarca el intento que ya estaba entregado.
pub fn clear_student_simulations(
    db: &DBEngine,
    student_id: i64,
) -> Result<(), sqlite::Error> {
    let mut statement = db.run_query(
        "
        UPDATE student_simulations
        SET selected = FALSE
        WHERE student_id = ?
        "
    )?;

    statement.bind((1, student_id))?;
    statement.next()?;

    Ok(())
}

/// Permite marcar una simulación como la entrega final, asegurando de forma atómica
/// que todas las demás simulaciones del mismo alumno queden desmarcadas.
pub fn select_student_simulation(
    db: &DBEngine,
    simulation_id: i64,
) -> Result<(), sqlite::Error> {
    // 1. Primero limpiamos todas las entregas para este alumno en particular
    // (Buscamos el student_id de forma anidada a través del id de la simulación)
    let mut clear_statement = db.run_query(
        "
        UPDATE student_simulations
        SET selected = FALSE
        WHERE student_id = (SELECT student_id FROM student_simulations WHERE id = ?)
        "
    )?;
    clear_statement.bind((1, simulation_id))?;
    clear_statement.next()?;

    // 2. Ahora sí, marcamos como TRUE el intento que el alumno eligió
    let mut select_statement = db.run_query(
        "
        UPDATE student_simulations
        SET selected = TRUE
        WHERE id = ?
        "
    )?;
    select_statement.bind((1, simulation_id))?;
    select_statement.next()?;

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

        let simulation_image_path: String = statement.read("simulation_image_path")?;

        simulations.push(StudentSimulation {
            id: statement.read("id")?,
            selected: statement.read::<i64, _>("selected")? == 1,
            attempt_number: statement.read("attempt_number")?,

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

            simulation_image_path: if simulation_image_path.is_empty() { None } else { Some(simulation_image_path) },
        });
    }

    Ok(simulations)
}