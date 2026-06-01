use gdal::{Dataset, raster::Buffer};

use crate::structs::depth_matrix::DepthMatrix;

type LoadGeotiffResult = Result<(Buffer<f64>, usize, usize, Option<f64>, f64, f64), gdal::errors::GdalError>;

// Carga los metadatos del geotiff en un Buffer y los retorna
fn load_geotiff(path: &str) -> LoadGeotiffResult{


    let dataset = Dataset::open(path)?;
    let geo_transform = dataset.geo_transform()?;
    
    // Extraemos las resoluciones (metros por píxel)
    let res_x = geo_transform[1];
    let res_y = geo_transform[5].abs();

    let (cols, rows) = dataset.raster_size();

    let raster_band = dataset.rasterband(1)?;

    let no_data = raster_band.no_data_value();

    let buffer: Buffer<f64> = raster_band.read_as(
        (0,0), 
        (cols,rows), 
        (cols,rows),
        None
    )?;

    Ok((buffer,cols,rows,no_data,res_x,res_y))
    
}

/// Saca los datos de profundidades del buffer y los guarda en una matriz (vector de vectores f64)
fn buffer_to_matrix(buffer: Buffer<f64>, cols: usize ) -> Vec<Vec<f64>>{

    let mut matrix: Vec<Vec<f64>> = Vec::new();
    let mut row = Vec::<f64>::new();

    let mut iterator = 1;

    for value in buffer.data(){

        row.push(*value);
        if iterator % cols == 0{

            matrix.push(row);
            row = Vec::<f64>::new();
        }

        iterator += 1;
    }

    matrix
}

/// procesa geoTIFF y devuelve una estructura DepthMatrix con toda la metadata del archivo necesaria para la simulacion
pub fn processing_geotiff(path: &str) -> Result<DepthMatrix, gdal::errors::GdalError>{
    
    let (buffer,cols, rows,no_data_value,size_x,size_y) = load_geotiff(path)?;

    let matrix: Vec<Vec<f64>> =  buffer_to_matrix(buffer, cols);

    Ok(
        DepthMatrix {
        data: matrix,
        width: cols,
        height: rows,
        no_data: no_data_value,
        size_x,
        size_y}
    )
}