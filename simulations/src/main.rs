use std::{fs::File, os::unix::process};
use std::io::Write;

use crate::processing::measuring::{MeasureMode, get_measures, get_points_circular_to_this, get_points_perpendicular_to_this};

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
        60.0, // azimut
        50.0, // separación
    );

    let puntos_a_medir = processing::measuring::find_measuring_points(&recorrido, 100.0);

    // --- Puntos perpendiculares ---
    let mut puntos_perpendiculares = Vec::new();
    let mut previous_point = &(puntos_a_medir[0]);
    let mut current_point = &(puntos_a_medir[0]);
    for punto in &puntos_a_medir {
        let group = get_points_perpendicular_to_this(Some(previous_point), current_point, Some(punto), 2.5);
        puntos_perpendiculares.extend(group);
        previous_point = current_point;
        current_point = punto;
    }
    let last_group = get_points_perpendicular_to_this(Some(previous_point), current_point, None, 2.5);
    puntos_perpendiculares.extend(last_group);


    // --- Puntos circulares ---
    let mut puntos_circulares = Vec::new();
    for punto in &puntos_a_medir {
        let group = get_points_circular_to_this(punto, 2.5);
        puntos_circulares.extend(group);
    }
 
    // --- Medidas ---
    let full_matrix_perp   = get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, &matrix, &puntos_a_medir);
    let full_matrix_circle = get_measures(MeasureMode::Circular { radius: 10.0 },            &matrix, &puntos_a_medir);

    // --- Escritura de archivos ---
 
    let mut res_file = File::create("res.txt").expect("No se pudo crear res.txt");
    for (x, y) in recorrido {
        writeln!(res_file, "({}, {})", x, y).expect("No se pudo escribir en res.txt");
    }
 
    let mut points_file = File::create("points.txt").expect("No se pudo crear points.txt");
    for (x, y) in &puntos_a_medir {
        writeln!(points_file, "({}, {})", x, y).expect("No se pudo escribir en points.txt");
    }
 
    let mut perp_file = File::create("perp.txt").expect("No se pudo crear perp.txt");
    for (x, y) in puntos_perpendiculares {
        writeln!(perp_file, "({}, {})", x, y).expect("No se pudo escribir en perp.txt");
    }
 
    let mut circ_file = File::create("circ.txt").expect("No se pudo crear circ.txt");
    for (x, y) in puntos_circulares {
        writeln!(circ_file, "({}, {})", x, y).expect("No se pudo escribir en circ.txt");
    }


    
    // Solo escribe puntos con medida distinta de 0
    let mut measures_perp_file = File::create("measures_perp.txt").expect("No se pudo crear measures_perp.txt");
    for row in 0..full_matrix_perp.len() {
        for col in 0..full_matrix_perp[row].len() {
            let val = full_matrix_perp[row][col];
            if val != 0.0 {
                writeln!(measures_perp_file, "{},{},{:.4}", col, row, val); // col=x, row=y
            }
        }
    }
 
    let mut measures_circle_file = File::create("measures_circle.txt").expect("No se pudo crear.");
    for row in 0..full_matrix_circle.len() {
        for col in 0..full_matrix_circle[row].len() {
            let val = full_matrix_circle[row][col];
            if val != 0.0 {
                // Escribimos primero columna (X) y luego fila (Y) para que Python lo lea bien
                writeln!(measures_circle_file, "{},{},{:.4}", col, row, val)
                    .expect("No se pudo escribir en measures_circle.txt");
            }
        }
    }

}
