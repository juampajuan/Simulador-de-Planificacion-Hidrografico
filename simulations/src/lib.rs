// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;
use common::{EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use crate::{processing::measuring::apply_disturbances, structs::{measurement_type::MeasurementsType, student_measuring_parameters::EchosounderLogic}};
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

    let distance_between_points = boat_speed*params.echo_sounder_parameters.pulse_repetition_interval/1000.0;

    let points_to_measure = processing::measuring::find_measuring_points(
        students_path,
        distance_between_points,
        matrix,
    );

    params.echo_sounder_parameters.create_echosounder();

    //Vec<((usize,usize), f64)>
    let measurements_points: MeasurementsType = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            get_measures(MeasureMode::Circular { angle: params.echo_sounder_parameters.angle }, matrix, &points_to_measure)
        },
        EcosondaMode::Multihaz => {
            get_measures(MeasureMode::Perpendicular {}, matrix, &points_to_measure)
        },
    };

    let mediciones_observadas = apply_disturbances(measurements_points, students_path, &params, matrix);

    interpolate(InterpolationMethod::GdalGrid(processing::gdal_grid_interp::GdalGridMethod::Linear), mediciones_observadas, matrix, distance_between_points)
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