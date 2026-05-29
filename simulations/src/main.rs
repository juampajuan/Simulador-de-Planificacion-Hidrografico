use std::fs::File;
use std::io::Write;
 
use crate::processing::measuring::{MeasureMode, get_measures, get_points_circular_to_this};
use crate::processing::images::makePng_with_matrix_and_interpolation;
use crate::processing::interpolation::{interpolate, InterpolationMethod};
use common::{EcosondaMode, EchosounderParameters};
use crate::structs::student_measuring_parameters::EchosounderLogic;
 
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
 
    let recorrido_distorcionado = processing::routing::generate_route(
        &matrix,
        90.0,  // azimut
        50.0,  // separación en metros
        5.0, //Max offset
    );
 
    // let puntos_a_medir = processing::measuring::find_measuring_points(&recorrido_distorcionado, 0.2, &matrix);

    // println!("size_x: {}, size_y: {}", matrix.size_x, matrix.size_y);
    // println!("width: {}, height: {}", matrix.width, matrix.height);

    // for point in puntos_a_medir{
    //     println!("{}, {}", point.0, point.1);
    // }

    // --- Puntos perpendiculares ---
    // let mut puntos_perpendiculares = Vec::new();
    // let mut previous_point = &(puntos_a_medir[0]);
    // let mut current_point = &(puntos_a_medir[0]);
    // for punto in &puntos_a_medir {
    //     let group = get_points_perpendicular_to_this(Some(previous_point), current_point, Some(punto), 2.5);
    //     puntos_perpendiculares.extend(group);
    //     previous_point = current_point;
    //     current_point = punto;
    // }
    // let last_group = get_points_perpendicular_to_this(Some(previous_point), current_point, None, 2.5);
    // puntos_perpendiculares.extend(last_group);

    // let medidas = processing::measuring::get_measures(MeasureMode::Circular { angle: (10.0) }, &matrix, &puntos_a_medir);

    // for i in 0..ancho{
    //     for j in 0..altura {
    //         println!("{}, {}. {}", i, j, medidas[i][j]);
    //     }
    // }
 
    // --- Ecosonda alta frecuencia (monohaz) ---
    let mut high_frequency = EchosounderParameters {
        mode: EcosondaMode::Monohaz,
        angle: 0.0,                      // se calcula en create_echosounder
        absortion_coefficient: 0.0,      // se calcula en create_echosounder
        max_limit: 100.0,
        min_limit: 0.0,
        pulse_repetition_interval: 100.0,
        pulse_length: 1,
        uses_high_frecuency: true,
        transmited_potency: 220.0,
        gain: 0.0,
        echosounder_velocity: 1450,
        threshold: 0.1,
    };
    high_frequency.create_echosounder();
 
    // --- Ecosonda baja frecuencia (monohaz) ---
    let mut low_frequency = EchosounderParameters {
        mode: EcosondaMode::Monohaz,
        angle: 0.0,
        absortion_coefficient: 0.0,
        max_limit: 100.0,
        min_limit: 0.0,
        pulse_repetition_interval: 100.0,
        pulse_length: 1,
        uses_high_frecuency: false,
        transmited_potency: 220.0,
        gain: 0.0,
        echosounder_velocity: 1450,
        threshold: 0.1,
    };
    low_frequency.create_echosounder();

 
    // --- Medidas (sin errores) ---
    // let full_matrix_high = get_measures(
    //     MeasureMode::Circular { angle: high_frequency.angle },
    //     &matrix,
    //     &puntos_a_medir,
    //     high_frequency.threshold,
    // );
 
    // let full_matrix_low = get_measures(
    //     MeasureMode::Circular { angle: low_frequency.angle },
    //     &matrix,
    //     &puntos_a_medir,
    //     low_frequency.threshold,
    // );
 
    //// --- Interpolación y guardado ---
    // let interpolacion = interpolate(InterpolationMethod::IDW, &puntos_a_medir, &full_matrix_high, &matrix);
    // makePng_with_matrix_and_interpolation(&interpolacion, &matrix);
 
    // --- Archivos de debug ---
    let mut res_file = File::create("res.txt").expect("No se pudo crear res.txt");
    for (x, y) in &recorrido_distorcionado {
        writeln!(res_file, "({}, {})", x, y).expect("No se pudo escribir en res.txt");
    }

    // let mut points_file = File::create("points.txt").expect("No se pudo crear points.txt");
    // for (x, y) in &puntos_a_medir {
    //     writeln!(points_file, "({}, {})", x, y).expect("No se pudo escribir en points.txt");
    // }
 
    // let mut perp_file = File::create("perp.txt").expect("No se pudo crear perp.txt");
    // for (x, y) in puntos_perpendiculares {
    //     writeln!(perp_file, "({}, {})", x, y).expect("No se pudo escribir en perp.txt");
    // }
 
    // let mut circ_file = File::create("circ.txt").expect("No se pudo crear circ.txt");
    // for punto in &puntos_a_medir {
    //     let group = get_points_circular_to_this(punto, high_frequency.angle, &matrix);
    //     for (x, y) in group {
    //         writeln!(circ_file, "({}, {})", x, y).expect("No se pudo escribir en circ.txt");
    //     }
    // }

    // // Solo escribe puntos con medida distinta de 0
    // let mut measures_perp_file = File::create("measures_perp.txt").expect("No se pudo crear measures_perp.txt");
    // for row in 0..full_matrix_perp.len() {
    //     for col in 0..full_matrix_perp[row].len() {
    //         let val = full_matrix_perp[row][col];
    //         if val != 0.0 {
    //             writeln!(measures_perp_file, "{},{},{:.4}", col, row, val); // col=x, row=y
    //         }
    //     }
    // }
 
    // let mut measures_circle_file = File::create("measures_circle.txt").expect("No se pudo crear.");
    // for row in 0..full_matrix_circle.len() {
    //     for col in 0..full_matrix_circle[row].len() {
    //         let val = full_matrix_circle[row][col];
    //         if val != 0.0 {
    //             // Escribimos primero columna (X) y luego fila (Y) para que Python lo lea bien
    //             writeln!(measures_circle_file, "{},{},{:.4}", col, row, val)
    //                 .expect("No se pudo escribir en measures_circle.txt");
    //         }
    //     }
    // }
}