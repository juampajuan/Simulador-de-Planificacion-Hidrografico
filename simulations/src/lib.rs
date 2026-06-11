// Asi se puede usar en servar.
pub mod structs;
use kiddo::fixed::distance;
use structs::depth_matrix::DepthMatrix;
use common::{EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use crate::{processing::measuring::apply_disturbances, structs::student_measuring_parameters::EchosounderLogic};
use image::{RgbImage};

use crate::{processing::{images::{makepng_with_matrix_and_path, makepng_with_matrix_and_interpolation}, interpolation::{InterpolationMethod, interpolate}, measuring::{MeasureMode, get_measures}, routing::generate_route}}; 

mod processing;


#[allow(clippy::result_unit_err)]
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

    let max_offset = match path_params.gnss_type {
        GnssType::NoCorrection => {20.0},
        GnssType::DGPSCorrection => {5.0},
        GnssType::PhaseCorrection => {1.0},
    };

    // Usar el struck path_params acá para generar ruta.
    generate_route(matrix, path_params.azimut, path_params.separacion, max_offset)

}

pub fn run_simulation(
    matrix: &DepthMatrix,
    students_path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
) -> Vec<Vec<f64>> {

    println!("Simulando...");

    let boat_speed = params.transport_parameters.speed;
    // pulse_repetition_interval en hz, pero necesito la distancia entre puntos, entonces lo paso a segundos y lo multiplico por la velocidad del barco.
    let distance_between_points = boat_speed * params.echo_sounder_parameters.pulse_repetition_interval.recip();

    let points_to_measure = processing::measuring::find_measuring_points(
        students_path,
        distance_between_points,
        matrix,
    );

    params.echo_sounder_parameters.create_echosounder();

    let measurements_ideal = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            get_measures(MeasureMode::Circular { angle: params.echo_sounder_parameters.angle }, matrix, &points_to_measure, params.echo_sounder_parameters.threshold)
        },
        EcosondaMode::Multihaz => {
            get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, matrix, &points_to_measure, params.echo_sounder_parameters.threshold)
        },
    };

    let mediciones_ideales: Vec<((usize, usize), f64)> = points_to_measure
        .iter()
        .map(|&p| (p, measurements_ideal[p.1][p.0]))
        .collect();

    let mediciones_observadas = apply_disturbances(mediciones_ideales, students_path, &params, matrix);

    let mut measurements_final = vec![vec![0.0f64; matrix.width]; matrix.height];
    let mut points_validos: Vec<(usize, usize)> = Vec::new();
    for (punto, z_obs) in &mediciones_observadas {
        if let Some(z) = z_obs {
            measurements_final[punto.1][punto.0] = *z;
            points_validos.push(*punto);
        }
    }

    interpolate(InterpolationMethod::Tin, &points_validos, &measurements_final, matrix, distance_between_points)
}

pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbImage  {
    println!("Generando PNG ...");

    makepng_with_matrix_and_path(matrix,path)
}

pub fn create_simulation_image(matrix: &DepthMatrix, student_interpolation: &Vec<Vec<f64>>) -> RgbImage {
    println!("Generando PNG ...");

    makepng_with_matrix_and_interpolation(student_interpolation, matrix)
}