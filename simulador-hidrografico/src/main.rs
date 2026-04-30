mod processing;
mod structs;

fn main() {

    let path = "Darsena_20cm_v2.tif";

    match processing::geotiff::processing_geotiff(path) {
        Ok(matrix) => matrix,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}
