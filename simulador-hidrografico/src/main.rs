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

    let recorrido = processing::routing::generate_route(300, &matrix);

    // Recorrido
    // println!("Recorrido:");
    for (x, y) in recorrido {
        println!("({}, {})", x, y); // cargo run > res.txt para guardar el resultado ahi, despues copio y pego eso en el graficador
    }

    // // Metadatos
    // println!("Ancho: {} pixels", matrix.data[0].len());
    // println!("Alto: {} pixels", matrix.data.len());
    // println!("No data value: {:?}", matrix.no_data);

    // // Primer valor de la matriz
    // println!("Primer pixel: {}", matrix.data[0][0]);
}
