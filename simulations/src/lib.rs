// Asi se puede usar en servar.
pub mod structs;
mod processing;
mod lib_helpers;
pub use structs::depth_matrix::DepthMatrix;
use common::{EcosondaMode, GnssType, PathParameters, StudentMeasuringParameters};
use processing::geotiff::GeotiffCoordinates;
use processing::interpolation::handler::interpolate;
use processing::measuring::{apply_disturbances, MeasureMode, get_measures}; 
use processing::images::{makepng_transparent_with_path, makepng_with_matrix_and_interpolation, make_shaded_png, create_scale_image, draw_covered_points, draw_path, COVERAGE_OVERLAY_COLOR};
use processing::routing::generate_route; 
use structs::{interpolation_type::InterpolationMethod, measurement_type::MeasurementsType, student_measuring_parameters::EchosounderLogic, simulation_constants::SimulationConstants};
use image::{RgbaImage};


/// Crea la matriz segun el tiff cargado para el alumno
/// Utiliza el crate processing::Geotiff
pub fn create_depth_matrix(file_path :&str, log_debug: &dyn Fn(&str)) -> Result<DepthMatrix, String> {
 
    log_debug("Generando depth_matrix...");
 
    match processing::geotiff::processing_geotiff(file_path) {
        Ok(matrix) => {
            log_debug("depth_matrix generada.");
            Ok(matrix)},

        Err(e) => Err(e.to_string()),
    }

}
/// Crea el recorrido segun los parametros indicados por el alumno
/// processing::routing
pub fn create_path(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters :f64, gnss_type: GnssType, log_debug: &dyn Fn(&str)) -> Vec<(usize,usize)> {

    log_debug("Generando recorrido...");

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

    let path= generate_route(matrix, path_params.azimut, path_params.separacion, max_offset);
    log_debug(&format!("Recorrido generado ({} puntos).", path.len()));

    path
}

/// Hace toda la logica de simulacion en base al recorrido del barco y los parametros seleccionados
/// Llama a los distintos crates que aplican errores e interpolan.
/// Devuelve la matriz interpolada
/// Utiliza processing::measuring,processing::interpolation
pub fn run_simulation(
    matrix: &DepthMatrix,
    students_path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
    constants: SimulationConstants,
    log_debug: &dyn Fn(&str),
    
) -> Result<Vec<Vec<f64>>, String> {

    log_debug("Simulando...");

    let boat_speed = params.transport_parameters.speed;
    let distance_between_points = boat_speed * params.echo_sounder_parameters.pulse_repetition_interval.recip();

    let points_to_measure = processing::measuring::find_measuring_points(
        students_path,
        distance_between_points,
        matrix,
    );

    log_debug(&format!("Puntos de medicion calculados ({}).", points_to_measure.len()));

    params.echo_sounder_parameters.create_echosounder(&constants);

    let measurements_points: MeasurementsType = match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            get_measures(MeasureMode::Circular { angle: params.echo_sounder_parameters.angle }, matrix, &points_to_measure)
        },
        EcosondaMode::Multihaz => {
            get_measures(MeasureMode::Perpendicular { angle: constants.echosounder.multihaz_angle_deg }, matrix, &points_to_measure)
        },
    };

    log_debug("Mediciones tomadas.");

    let mediciones_observadas = apply_disturbances(measurements_points, students_path, &params, matrix, &constants);

    log_debug("Errores aplicados a las mediciones.");

    let resultado = interpolate(InterpolationMethod::GdalTin, mediciones_observadas, matrix);

    if resultado.is_ok() {
        log_debug("Interpolacion completada.");
    }

    resultado
}

/// Devuelve el PNG del recorrido hecho por el barco segun los parametros del alumno
/// Utiliza processing::images
pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
    log_debug: &dyn Fn(&str),
)-> RgbaImage  {

    log_debug("Generando PNG de recorrido...");
    let img = makepng_transparent_with_path(matrix, path);
    log_debug("PNG de recorrido generado.");
    img
}

/// Devuelve el PNG de la matriz interpolada con la escala de colores correspondiente
/// Utiliza processing::images
pub fn create_simulation_image(matrix: &DepthMatrix, student_interpolation: &[Vec<f64>], log_debug: &dyn Fn(&str)) -> (RgbaImage, f64, f64) {

    log_debug("Generando PNG de simulacion...");
    let result = makepng_with_matrix_and_interpolation(student_interpolation, matrix);
    log_debug("PNG de simulacion generado.");
    result
}

pub fn create_scale_pure_image(log_debug: &dyn Fn(&str)) -> RgbaImage {

    log_debug("Generando escala...");
    create_scale_image()
}

/// Genera un png que muestra la cobertura segun el tipo de medicion seleccionada por el alumno.
/// Utiliza processing::measures, processing::images
pub fn create_path_with_coverage(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
    constants: SimulationConstants,
    log_debug: &dyn Fn(&str),
) -> RgbaImage {

    log_debug("Generando PNG con cobertura...");
 
    // false = radio uniforme por profundidad promedio, para no espoilear el resultado
    let covered_points = lib_helpers::get_covered_points(matrix, path, &mut params, false, constants);
    log_debug("Cobertura calculada.");
 
    let img = make_shaded_png(matrix, &covered_points, path);
    log_debug("PNG con cobertura generado.");
    img
}

pub fn create_simulation_with_coverage(
    matrix: &DepthMatrix,
    student_interpolation: &[Vec<f64>],
    path: &Vec<(usize, usize)>,
    mut params: StudentMeasuringParameters,
    constants: SimulationConstants,
    log_debug: &dyn Fn(&str),
) -> (RgbaImage, f64, f64) {

    log_debug("Generando PNG de simulacion con cobertura...");
 
    let (mut img, min_val, max_val) = makepng_with_matrix_and_interpolation(student_interpolation, matrix);
    // true = radio real por punto, el resultado ya esta a la vista igual
    let covered_points = lib_helpers::get_covered_points(matrix, path, &mut params, true, constants);
    log_debug("Cobertura calculada.");
    draw_covered_points(&mut img, &covered_points, COVERAGE_OVERLAY_COLOR);

    draw_path(&mut img, matrix, path, image::Rgba([255, 255, 255, 180]));

    log_debug("PNG de simulacion con cobertura generado.");

    (img, min_val, max_val)
}

/// Obtiene las cordenadas del centro y las esquinas del geotiff y las devuelve
/// Utiliza processing::Geotiff.
pub fn get_geotiff_corners(
    file_path: &str,
    log_debug: &dyn Fn(&str),
) -> GeotiffCoordinates {
    log_debug(&format!("Calculando coordenadas del geotiff {file_path}..."));
    // (sup_izq, sup_der, inf_izq, inf_der, centro), cada uno (lat, lon)
    let coordinates = match processing::geotiff::get_geotiff_coordinates(file_path) {
        Ok(coordinates) => coordinates,
        Err(e) => {
            return Err(e);
        }
    };

    log_debug("Coordenadas calculadas.");

    Ok(coordinates)
}