use std::{fs::File, os::unix::process};
use std::io::Write;

mod processing;
mod structs;

fn main() {

    let path = "Darsena_20cm_v2.tif";

    let matrix = match processing::geotiff::processing_geotiff(path) {
        Ok(matrix) => matrix,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let recorrido = processing::routing::generate_route(
        &matrix,
        127.0, // azimut
        50.0, // separación
    );

    let (puntos_a_medir, puntos_perpendiculares) = processing::measuring::find_measuring_points(&recorrido, 100.0);

    // println!("Resolución X: {}, Y: {}", matrix.size_x, matrix.size_y);
    // println!("Ancho: {}, Alto: {}", matrix.width, matrix.height);
    // Recorrido
    // println!("Recorrido:");
    let mut res_file = File::create("res.txt").expect("No se pudo crear res.txt"); 
    for (x, y) in recorrido {
        writeln!(res_file, "({}, {})", x, y).expect("No se pudo escribir en res.txt");
    }
    let mut points_file = File::create("points.txt").expect("No se pudo crear points.txt");
    for (x, y) in puntos_a_medir {
        writeln!(points_file, "({}, {})", x, y).expect("No se pudo escribir en points.txt");
    }
    let mut points_file = File::create("perp.txt").expect("No se pudo crear points.txt");
    for (x, y) in puntos_perpendiculares {
        writeln!(points_file, "({}, {})", x, y).expect("No se pudo escribir en points.txt");
    }
    // // Metadatos
    // println!("Ancho: {} pixels", matrix.data[0].len());
    // println!("Alto: {} pixels", matrix.data.len());
    // println!("No data value: {:?}", matrix.no_data);

    // // Primer valor de la matriz
    // println!("Primer pixel: {}", matrix.data[0][0]);
}
