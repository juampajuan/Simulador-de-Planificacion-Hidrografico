// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;
use common::{EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use crate::{processing::{interpolation::interpolation_handler:: interpolate, measuring::apply_disturbances}, structs::{interpolation_type::InterpolationMethod, measurement_type::MeasurementsType, student_measuring_parameters::EchosounderLogic}};
use image::{RgbaImage};
use crate::{processing::{images::{makepng_with_matrix_and_path, makepng_with_matrix_and_interpolation, make_shaded_png}, measuring::{MeasureMode, get_measures}, routing::generate_route}}; 

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

    let measurements_points: MeasurementsType = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            get_measures(MeasureMode::Circular { angle: params.echo_sounder_parameters.angle }, matrix, &points_to_measure)
        },
        EcosondaMode::Multihaz => {
            get_measures(MeasureMode::Perpendicular {}, matrix, &points_to_measure)
        },
    };

    let mediciones_observadas = apply_disturbances(measurements_points, students_path, &params, matrix);

    interpolate(InterpolationMethod::GdalTin, mediciones_observadas, matrix)
}

pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbaImage  {
    println!("Generando PNG ...");

    makepng_with_matrix_and_path(matrix,path)
}

pub fn create_simulation_image(matrix: &DepthMatrix, student_interpolation: &Vec<Vec<f64>>) -> RgbaImage {
    println!("Generando PNG ...");

    makepng_with_matrix_and_interpolation(student_interpolation, matrix)
}

pub fn create_path_with_shadows(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
    params: StudentMeasuringParameters,
)-> RgbaImage{
    println!("Generando PNG con sombras ...");

    let (sum, count) = matrix.data
    .iter()
    .flatten()
    .filter(|&&v| v.is_finite())
    .fold((0.0f64, 0usize), |(s, c), &v| (s + v, c + 1));

    let avg = sum / count as f64;

    let width: f64 = match params.echo_sounder_parameters.mode{
        EcosondaMode::Monohaz => {
            avg * (params.echo_sounder_parameters.angle.to_radians()/2.0).tan()
        },
        EcosondaMode::Multihaz => {
            let angle_deg:f64 = 60.0;
            (2.0*(avg)*(angle_deg.to_radians()).tan())/2.0
        },
    };

    make_shaded_png(matrix, path, width)
}