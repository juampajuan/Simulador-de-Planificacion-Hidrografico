// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;
use structs::student_path_parameters::StudentPathParameters;
use structs::gnss_type::GnssType;
use common::{EchosounderParameters, EcosondaMode, StudentMeasuringParameters};
use image::{RgbImage};

use crate::{processing::{images::{makePNG_with_matrix_and_path, makePng_with_matrix_and_interpolation}, interpolation::{InterpolationMethod, interpolate}, measuring::{MeasureMode, get_measures}, routing::generate_route}, structs::{echosonder::EcosondaMode, student_measuring_parameters::StudentMeasuringParameters}}; 

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

pub fn create_path(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters :f64, gnss_type: String) -> Vec<(usize,usize)> {

    println!("Generando recorrido ...");

    let path_params = StudentPathParameters {
        azimuth_deg,
        separation_meters,
        gnss_type: match gnss_type.as_str() {
            "Corrección de Fase" => GnssType::PhaseCorrection,
            "Corrección DGPS" => GnssType::DGPSCorrection,
            _ => GnssType::NoCorrection, 
        }
    };

    // Usar el struck path_params acá para generar ruta.
    let path = generate_route(matrix, path_params.azimuth_deg, path_params.separation_meters);

    path

}

pub fn run_simulation(
    matrix: &DepthMatrix,
    students_path: &Vec<(usize, usize)>,
    params: StudentMeasuringParameters,
) -> Vec<Vec<f64>> {

    println!("Simulando ...");

    let v_real = 1500.0;
    let mut echo = params.echo_sounder_parameters;

    echo.create_echosounder();

    let mode = match echo.mode {
        Some(m) => m,
        None => panic!("Llamar create_echosounder() antes de run_simulation()"),
    };

    //Calcula los intervalos entre cada medicion en base a la velociad del barco (metros/ms) y el intervalo de repeticion de plso (ms)
    let distance_between_points = params.boat.speed/echo.pulse_repetition_interval;

    let points_to_measure = processing::measuring::find_measuring_points(
        students_path,
        distance_between_points,
        &matrix,
    );

    //calcular umbral en base a los parametros del alumno
    let measurements_ideal = match mode {
        EcosondaMode::Monohaz {angle, ..}=> {
            get_measures(MeasureMode::Circular { angle: angle }, &matrix, &points_to_measure, echo.threshold)
        },
        EcosondaMode::Multihaz=> {
            get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, &matrix, &points_to_measure, echo.threshold)
        },
    };

    let mediciones_ideales: Vec<((usize, usize), f64)> = points_to_measure
        .iter()
        .map(|&p| (p, measurements_ideal[p.1][p.0]))
        .collect();

    let mediciones_observadas = echo.apply_errors(
        mediciones_ideales,
        v_real,
        params.uses_sound_profiler,
    );

    let mut measurements_final = vec![vec![0.0f64; matrix.width]; matrix.height];
    let mut points_validos: Vec<(usize, usize)> = Vec::new();
    for (punto, z_obs) in &mediciones_observadas {
        if let Some(z) = z_obs {
            measurements_final[punto.1][punto.0] = *z;
            points_validos.push(*punto);
        }
    }

    interpolate(InterpolationMethod::IDW, &points_validos, &measurements_final, &matrix)
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