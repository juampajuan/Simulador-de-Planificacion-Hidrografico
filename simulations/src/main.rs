use std::{fs::File, os::unix::process};
use std::io::Write;

use crate::processing::measuring::{MeasureMode, get_measures, get_points_circular_to_this, get_points_perpendicular_to_this};
use crate::processing::images::{makePng_with_matrix_and_interpolation};
use crate::processing::interpolation::{interpolate,InterpolationMethod };
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
    
    // let altura: usize = matrix.height;
    // let ancho: usize = matrix.width;


    let recorrido = processing::routing::generate_route(
        &matrix,
        90.0, // azimut
        50.0, // separación
    );


    // for point in recorrido{
    //     println!("{}, {}", point.0, point.1);
    // }

    //x,y
    //2045, 665

    let puntos_a_medir = processing::measuring::find_measuring_points(&recorrido, 100.0, &matrix);

    
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


    // --- Puntos circulares ---
    let mut puntos_circulares = Vec::new();
    for punto in &puntos_a_medir {
        let group = get_points_circular_to_this(punto, 20.0, &matrix);
        puntos_circulares.extend(group);
    }
 
    // --- Medidas ---

    let mut highfrecuency = EchosounderParameters{
        max_limit: 100.0,
        min_limit: 0.0,
        pulse_repetition_interval:100.0,
        pulse_length:1,
        uses_high_frecuency:true,
        transmited_potency: 220.0,
        gain: 0.0,
        echosounder_velocity: 1450,
        uses_monohaz: true,
        threshold: 0.1,
        mode: None,
    };

    highfrecuency.create_echosounder();

    let mut lowfrecuency = EchosounderParameters{
        max_limit: 100.0,
        min_limit: 0.0,
        pulse_repetition_interval:100.0,
        pulse_length:1,
        uses_high_frecuency:false,
        transmited_potency: 220.0,
        gain: 0.0,
        echosounder_velocity: 1450,
        uses_monohaz: true,
        mode: None,
        threshold: 0.1,
    };

    lowfrecuency.create_echosounder();

    let real_sound_velocity: f64 = 1500.0;

    
    let full_matrix_circle_high_frecuency = match highfrecuency.mode {
        Some(EcosondaMode::Monohaz {angle, ..}) => {
            get_measures(MeasureMode::Circular { angle: angle }, &matrix, &puntos_a_medir, highfrecuency.threshold)
        },
        _ =>{
            vec![]
        }
    };

    let full_matrix_circle_low_frecuency = match lowfrecuency.mode {
        Some(EcosondaMode::Monohaz {angle, ..}) => {
            get_measures(MeasureMode::Circular { angle: angle }, &matrix, &puntos_a_medir, lowfrecuency.threshold)
        },
        _ =>{
            vec![]
        }

    };

    // let full_matrix_perp   = get_measures(MeasureMode::Perpendicular { step_distance: 2.5 }, &matrix, &puntos_a_medir);
    // // --- Escritura de archivos ---

    let interpolacion = interpolate(InterpolationMethod::IDW,&puntos_a_medir, &full_matrix_circle_high_frecuency, &matrix);

    makePng_with_matrix_and_interpolation(&interpolacion, &matrix);
    
    let mut res_file = File::create("res.txt").expect("No se pudo crear res.txt");
    for (x, y) in recorrido {
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
 
    let mut circ_file = File::create("circ.txt").expect("No se pudo crear circ.txt");
    for (x, y) in puntos_circulares {
        writeln!(circ_file, "({}, {})", x, y).expect("No se pudo escribir en circ.txt");
    }


    
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
