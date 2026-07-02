// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;
use common::{EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use crate::{processing::{geotiff::{GeotiffCoordinates, get_matrix_avg_depth}, interpolation::interpolation_handler:: interpolate, measuring::{apply_disturbances, beam::calculate_covered_radius}}, structs::{interpolation_type::InterpolationMethod, measurement_type::MeasurementsType, student_measuring_parameters::EchosounderLogic}};
use image::{RgbaImage};
use crate::{processing::{images::{makepng_transparent_with_path, makepng_with_matrix_and_interpolation, make_shaded_png, create_scale_image}, measuring::{MeasureMode, get_measures}, routing::generate_route}}; 
mod processing;


/// Crea la matriz segun el tiff cargado para el alumno
/// Utiliza el crate processing::Geotiff
#[allow(clippy::result_unit_err)]
pub fn create_depth_matrix(file_path :&str) -> Result<DepthMatrix,()>{

    println!("Generando depth_matrix ...");

    let matrix = match processing::geotiff::processing_geotiff(file_path) {
        Ok(matrix) => matrix,
        Err(e) => {
            println!("Error: {}", e);
            return Err(());
        }
    };

    Ok(matrix) 
}

/// Crea el recorrido segun los parametros indicados por el alumno
/// processing::routing
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

    generate_route(matrix, path_params.azimut, path_params.separacion, max_offset)
}

/// Hace toda la logica de simulacion en base al recorrido del barco y los parametros seleccionados
/// Llama a los distintos crates que aplican errores e interpolan.
/// Devuelve la matriz interpolada
/// Utiliza processing::measuring,processing::interpolation
pub fn run_simulation(
    matrix: &DepthMatrix,
    students_path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
) -> Vec<Vec<f64>> {

    println!("Simulando...");

    let boat_speed = params.transport_parameters.speed;
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

/// Devuelve el PNG del recorrido hecho por el barco segun los parametros del alumno
/// Utiliza processing::images
pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbaImage  {
    println!("Generando PNG ...");

    makepng_transparent_with_path(matrix, path)
}

/// Devuelve el PNG de la matriz interpolada con la escala de colores correspondiente
/// Utiliza processing::images
pub fn create_simulation_image(matrix: &DepthMatrix, student_interpolation: &[Vec<f64>]) -> (RgbaImage, f64, f64) {
    println!("Generando PNG ...");

    makepng_with_matrix_and_interpolation(student_interpolation, matrix)
}

pub fn create_scale_pure_image() -> RgbaImage {
    println!("Generando escala ...");
    create_scale_image()
}

/// Genera un png que muestra la cobertura segun el tipo de medicion seleccionada por el alumno.
/// Utiliza processing::measures, processing::images
pub fn create_path_with_shadows(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
) -> RgbaImage {
    println!("Generando PNG con sombras ...");

    let boat_speed = params.transport_parameters.speed;
    let distance_between_points = boat_speed * params.echo_sounder_parameters.pulse_repetition_interval.recip();

    let points_to_measure = processing::measuring::find_measuring_points(
        path,
        distance_between_points,
        matrix,
    );

    params.echo_sounder_parameters.create_echosounder();

    let avg_depth = get_matrix_avg_depth(&(matrix)).unwrap_or(0.0);

    let covered_points: Vec<((usize, usize), f64)> = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            // Para monohaz mostramos todos los píxeles dentro del círculo del haz,
            // no solo el punto central — así se ve el área real cubierta.
            let mut covered = Vec::new();
            let radius = calculate_covered_radius(avg_depth, params.echo_sounder_parameters.angle, matrix);

            for &point in &points_to_measure {
                let circle_points = processing::measuring::beam::get_points_in_radius(
                    &point,
                    radius,
                    matrix,
                );

                for p in circle_points {
                    covered.push((p, matrix.data[p.1][p.0]));
                }
            }
            covered
        },
        EcosondaMode::Multihaz => {
            let measurements = get_measures(MeasureMode::Perpendicular {}, matrix, &points_to_measure);
            match measurements {
                MeasurementsType::Multihaz { central_measurments, paralel_measurment_1, paralel_measurment_2 } => {
                    let mut all = central_measurments;
                    all.extend(paralel_measurment_1);
                    all.extend(paralel_measurment_2);
                    all
                },
                MeasurementsType::Monohaz { measurements } => measurements,
            }
        },
    };

    make_shaded_png(matrix, &covered_points, path)
}

/// Obtiene las cordenadas del centro y las esquinas del geotiff y las devuelve
/// Utiliza processing::Geotiff.
pub fn get_geotiff_corners(
    file_path: &str,
) -> GeotiffCoordinates {
    // (sup_izq, sup_der, inf_izq, inf_der, centro), cada uno (lat, lon)
    let coordinates = match processing::geotiff::get_geotiff_coordinates(file_path) {
        Ok(coordinates) => coordinates,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(coordinates)
}