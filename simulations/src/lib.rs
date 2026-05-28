// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;
use common::{EchosounderParameters, EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use crate::structs::student_measuring_parameters::EchosounderLogic;
use image::{RgbImage};

use crate::{processing::{images::{makePNG_with_matrix_and_path, makePng_with_matrix_and_interpolation}, interpolation::{InterpolationMethod, interpolate}, measuring::{MeasureMode, get_measures}, routing::generate_route}}; 

mod processing;

// TODO: Hay q completar. Decidir el Result q retorna

pub fn create_depth_matrix(file_path :&str) -> Result<DepthMatrix,()>{

    println!("Generando depth_matrix ...");

    let matrix = match processing::geotiff::processing_geotiff(file_path) {
        Ok(matrix) => matrix,
        Err(e) => {
            println!("Error: {}", e);
            return Err(());  // ← Retorna el Err a la función
        }
    };

    Ok(matrix) 
}

pub fn create_path(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters :f64, gnss_type: GnssType) -> Vec<(usize,usize)> {

    println!("Generando recorrido ...");

    let path_params = PathParameters {
        azimut: azimuth_deg,
        separacion: separation_meters,
        gnss_type,
    };

    // Usar el struck path_params acá para generar ruta.
    let path = generate_route(matrix, path_params.azimut, path_params.separacion);

    path

}

pub fn run_simulation(
    matrix: &DepthMatrix,
    students_path: &Vec<(usize, usize)>,
    params: StudentMeasuringParameters,
) -> Vec<Vec<f64>> {

    println!("Simulannnnndo ...");

    println!("Echo sounder a crear ...");
    let v_real = 1500.0;
    let mut echo = params.echo_sounder_parameters;

    println!("Echo sounder creado ...");

    let boat_speed = match params.boat {
        common::Boat::W { speed } => speed,
        common::Boat::Y { speed } => speed,
    };

    //Calcula los intervalos entre cada medicion en base a la velociad del barco (metros/ms) y el intervalo de repeticion de plso (ms)
    // let distance_between_points = boat_speed/echo.pulse_repetition_interval;
    let distance_between_points = 100.0;

    let points_to_measure = processing::measuring::find_measuring_points(
        students_path,
        distance_between_points,
        &matrix,
    );

    let mut echo = params.echo_sounder_parameters;
    echo.create_echosounder();

    println!("{:?}", params);
    println!("{:?}", echo);


//  StudentMeasuringParameters { uses_mathegapher: false, uses_sound_profiler: true, uses_inertial_sensor: false, echo_sounder_parameters: EchosounderParameters { mode: Monohaz { angle: 0.0, absortion_coefficient: 0.0 }, max_limit: 100.0, min_limit: 0.0, pulse_repetition_interval: 100.0, pulse_length: 1, uses_high_frecuency: true, transmited_potency: 220.0, gain: 0.0, echosounder_velocity: 1555, threshold: 0.1 }, boat: W { speed: 100.0 } }
//  EchosounderParameters { mode: Monohaz { angle: 0.0, absortion_coefficient: 0.0 }, max_limit: 100.0, min_limit: 0.0, pulse_repetition_interval: 100.0, pulse_length: 1, uses_high_frecuency: true, transmited_potency: 220.0, gain: 0.0, echosounder_velocity: 1555, threshold: 0.1 }

    //calcular umbral en base a los parametros del alumno
    let measurements_ideal = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            get_measures(MeasureMode::Circular { angle: echo.angle }, &matrix, &points_to_measure, echo.threshold)
        },
        EcosondaMode::Multihaz => {
            get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, &matrix, &points_to_measure, echo.threshold)
        },
    };

    interpolate(InterpolationMethod::IDW, &points_to_measure, &measurements_ideal, &matrix)

    // let mediciones_ideales: Vec<((usize, usize), f64)> = points_to_measure
    //     .iter()
    //     .map(|&p| (p, measurements_ideal[p.1][p.0]))
    //     .collect();

    // let mediciones_observadas = echo.apply_errors(
    //     mediciones_ideales,
    //     v_real,
    //     params.uses_sound_profiler,
    // );

    // let mut measurements_final = vec![vec![0.0f64; matrix.width]; matrix.height];
    // let mut points_validos: Vec<(usize, usize)> = Vec::new();
    // for (punto, z_obs) in &mediciones_observadas {
    //     if let Some(z) = z_obs {
    //         measurements_final[punto.1][punto.0] = *z;
    //         points_validos.push(*punto);
    //     }
    // }

    
}

pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbImage  {
    println!("Generando PNG ...");

    let img = makePNG_with_matrix_and_path(matrix,path);

    img
}

pub fn create_simulation_image(matrix: &DepthMatrix, student_interpolation: &Vec<Vec<f64>>) -> RgbImage {
    println!("Generando PNG ...");

    let img = makePng_with_matrix_and_interpolation(student_interpolation, matrix);
    img
}