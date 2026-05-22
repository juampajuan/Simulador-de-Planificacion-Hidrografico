// Asi se puede usar en servar.
pub mod structs;
use structs::depth_matrix::DepthMatrix;

use image::{RgbImage};

use crate::{processing::{images::makePNG_with_matrix_and_path, interpolation::interpolacion_jullen_theorem, measuring::{MeasureMode, get_measures}, routing::generate_route}, structs::echosonder::EcosondaMode}; 

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

pub fn create_path(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters :f64) -> Vec<(usize,usize)> {

    println!("Generando recorrido ...");

    let path = generate_route(matrix, azimuth_deg, separation_meters);

    path

}

pub fn run_simulation(matrix: &DepthMatrix, students_path: &Vec<(usize, usize)>, distance_between_points:f64, mode: EcosondaMode  ) -> Vec<Vec<f64>>{

    println!("Simulando ...");

    let points_to_measure = processing::measuring::find_measuring_points(students_path, distance_between_points);

    let measurements = match mode {
        EcosondaMode::Monohaz => get_measures(MeasureMode::Circular { radius: 10.0 }, &matrix, &points_to_measure),
        EcosondaMode::Multihaz => get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, &matrix, &points_to_measure),
    };

    let interpolacion = interpolacion_jullen_theorem(&points_to_measure, &measurements);
    
    interpolacion

}

pub fn create_path_image(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
)-> RgbImage  {
    println!("Generando PNG ...");

    let img = makePNG_with_matrix_and_path(matrix,path);

    img
}

pub fn create_simulation_image(/* Struct del alumno o lo que me digan */){
    println!("Generando PNG ...");
}