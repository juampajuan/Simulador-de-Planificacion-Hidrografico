use gdal::{Dataset, raster::Buffer};

use crate::structs::depth_matrix::DepthMatrix;

fn load_geotiff(path: &str) -> Result<(Buffer<f64>,usize,usize, Option<f64>), gdal::errors::GdalError>{

    // Loads geoTIFF metadata into a Buffer Type. Then returns it.

    let dataset = Dataset::open(path)?;

    let (cols, rows) = dataset.raster_size();

    let raster_band = dataset.rasterband(1)?;

    let no_data = raster_band.no_data_value();

    let buffer: Buffer<f64> = raster_band.read_as(
        (0,0), 
        (cols,rows), 
        (cols,rows),
        None
    )?;

    Ok((buffer,cols,rows,no_data))
    
}

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

        iterator = iterator + 1;
    }

    matrix
}

pub fn processing_geotiff(path: &str) -> Result<DepthMatrix, gdal::errors::GdalError>{
    
    // process geoTIFF and return a DepthMatrix Type with all the metadata of the file


    let (buffer,cols, rows,no_data_value) = load_geotiff(path)?;

    let matrix: Vec<Vec<f64>> =  buffer_to_matrix(buffer, cols);

    Ok(
        DepthMatrix {
        data: matrix,
        width: cols,
        heigth: rows,
        no_data: no_data_value}
    )
}