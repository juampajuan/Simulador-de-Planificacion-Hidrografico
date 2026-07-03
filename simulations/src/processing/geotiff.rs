use gdal::{Dataset, raster::Buffer, spatial_ref::{AxisMappingStrategy, CoordTransform, SpatialRef}};

use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::processing_helpers::is_valid;

type LoadGeotiffResult = Result<(Buffer<f64>, usize, usize, Option<f64>, f64, f64, [f64; 6], String), gdal::errors::GdalError>;
pub type GeotiffCoordinates = Result<((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64)), gdal::errors::GdalError>;

// Carga los metadatos del geotiff en un Buffer y los retorna
fn load_geotiff(path: &str) -> LoadGeotiffResult{


    let dataset = Dataset::open(path)?;
    let geo_transform = dataset.geo_transform()?;
    let projection = dataset.projection();

    let spatial_ref = SpatialRef::from_definition(&projection)
        .map_err(|_| gdal::errors::GdalError::BadArgument("No se pudo leer el sistema de coordenadas".to_string()))?;

    if spatial_ref.is_geographic() {
        return Err(gdal::errors::GdalError::BadArgument(
            "El GeoTIFF tiene coordenadas geográficas (grados). Se requieren coordenadas proyectadas (metros).".to_string()
        ));
    }

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

    Ok((buffer,cols,rows,no_data,res_x,res_y,geo_transform,projection))
    
}

/// Saca los datos de profundidades del buffer y los guarda en una matriz (vector de vectores f64)
fn buffer_to_matrix(buffer: Buffer<f64>, cols: usize ) -> Vec<Vec<f64>>{

    let mut matrix: Vec<Vec<f64>> = Vec::new();
    let mut row = Vec::<f64>::new();

    let mut iterator = 1;

    #[allow(clippy::explicit_counter_loop)]
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
    
    let (buffer,cols, rows,no_data_value,size_x,size_y,geo_transform,projection) = load_geotiff(path)?;

    let matrix: Vec<Vec<f64>> =  buffer_to_matrix(buffer, cols);

    Ok(
        DepthMatrix {
        data: matrix,
        width: cols,
        height: rows,
        no_data: no_data_value,
        size_x,
        size_y,
        geo_transform,
        projection}
    )
}

/// Calcula con el geotrasform las coordenadas segun el pixel de la matriz
fn calculate_coordinate(gt : [f64; 6],col: f64, row: f64) -> (f64,f64) {
    (gt[0] + col * gt[1] + row * gt[2] ,  gt[3] + col * gt[4] + row * gt[5])
}

/// Devuelve las coordenadas del centro y de las esquinas del TIFF
/// Esto es para el mapa del fondo
pub fn get_geotiff_coordinates(path: &str) -> GeotiffCoordinates {

    let dataset = Dataset::open(path)?;
    let geo_transform = dataset.geo_transform()?;
    let projection = dataset.projection();
    
    let (cols, rows) = dataset.raster_size();
    let (w, h) = (cols as f64, rows as f64);

    let ul = calculate_coordinate(geo_transform,0.0, 0.0);     // superior izquierda
    let ur = calculate_coordinate(geo_transform,w, 0.0);     // superior derecha
    let ll = calculate_coordinate(geo_transform,0.0, h);       // inferior izquierda
    let lr = calculate_coordinate(geo_transform,w, h);       // inferior derecha
    let ce = calculate_coordinate(geo_transform,w / 2.0, h / 2.0); // centro

    // reproyectar de proyectado -> lat/lon (WGS84)
    let mut src = SpatialRef::from_definition(&projection)?;
    let mut dst = SpatialRef::from_epsg(4326)?;
    src.set_axis_mapping_strategy(AxisMappingStrategy::TraditionalGisOrder);
    dst.set_axis_mapping_strategy(AxisMappingStrategy::TraditionalGisOrder);
    let ct = CoordTransform::new(&src, &dst)?;

    let mut xs = [ul.0, ur.0, ll.0, lr.0, ce.0];
    let mut ys = [ul.1, ur.1, ll.1, lr.1, ce.1];
    let mut zs = [0.0; 5];
    ct.transform_coords(&mut xs, &mut ys, &mut zs)?;

    //(latitud, longitud).
    Ok((
        (ys[0], xs[0]),
        (ys[1], xs[1]),
        (ys[2], xs[2]),
        (ys[3], xs[3]),
        (ys[4], xs[4]),
    ))
}

pub fn get_matrix_avg_depth(matrix: &DepthMatrix) -> Option<f64> {
    let mut sum = 0.0;
    let mut count = 0;

    for row in &matrix.data { 
        for value in row { 
            if is_valid(*value, matrix){
                sum += *value;
                count += 1;
            }
        }
    }

    if count == 0 {
        None
    } else {
        Some(sum / count as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
    // Geotransform de ejemplo: origen en (1000, 2000), pixel de 0.2m,
    // sin rotacion (caso comun para nuestros geotiffs).
    const GT: [f64; 6] = [1000.0, 0.2, 0.0, 2000.0, 0.0, -0.2];
 
    #[test]
    fn esquina_superior_izquierda_es_el_origen() {
        // col=0, row=0 tiene que devolver exactamente el origen del geotransform.
        let (x, y) = calculate_coordinate(GT, 0.0, 0.0);
        assert_eq!(x, 1000.0);
        assert_eq!(y, 2000.0);
    }
 
    #[test]
    fn avanza_en_x_segun_el_ancho_de_pixel() {
        // 10 columnas * 0.2m/px = 2m de avance en x, sin tocar y.
        let (x, y) = calculate_coordinate(GT, 10.0, 0.0);
        assert_eq!(x, 1002.0);
        assert_eq!(y, 2000.0);
    }
 
    #[test]
    fn avanza_en_y_hacia_abajo_porque_el_alto_de_pixel_es_negativo() {
        // 10 filas * -0.2m/px = -2m: y decrece a medida que bajamos en la imagen,
        // que es justo la convencion de los geotiffs (fila 0 = arriba).
        let (x, y) = calculate_coordinate(GT, 0.0, 10.0);
        assert_eq!(x, 1000.0);
        assert_eq!(y, 1998.0);
    }
}

