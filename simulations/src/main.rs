use std::os::unix::process;

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
        50.0, // azimut
        50.0, // separación
    );

    let puntos_a_medir = processing::measuring::find_measuring_points(&recorrido, 100.0);

    // println!("Resolución X: {}, Y: {}", matrix.size_x, matrix.size_y);
    // println!("Ancho: {}, Alto: {}", matrix.width, matrix.height);
    // Recorrido
    // println!("Recorrido:");
    for (x, y) in recorrido {
        println!("({}, {})", x, y); // cargo run > res.txt para guardar el resultado ahi, despues copio y pego eso en el graficador
    }

    // for (x, y) in puntos_a_medir{
    //     println!("({}, {})", x, y);
    // }

    // // Metadatos
    // println!("Ancho: {} pixels", matrix.data[0].len());
    // println!("Alto: {} pixels", matrix.data.len());
    // println!("No data value: {:?}", matrix.no_data);

    // // Primer valor de la matriz
    // println!("Primer pixel: {}", matrix.data[0][0]);
}
