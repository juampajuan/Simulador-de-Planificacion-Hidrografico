use std::fs;
use std::path::PathBuf;
use std::process::Command;
 
use gdal::{Dataset, DatasetOptions, GdalOpenFlags};
 
use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::geotiff::processing_geotiff;

/// Se encarga de borrar los 3 archivos temporales de esta llamada al salir
/// de la funcion -- sea porque termino bien, o porque cualquiera de los `?`
struct TempFilesGuard {
    csv_path: PathBuf,
    vrt_path: PathBuf,
    tif_path: PathBuf,
}

impl Drop for TempFilesGuard {
    fn drop(&mut self) {
        // Puede que alguno todavia no se haya llegado a crear -- remove_file
        // en ese caso da error, lo ignoramos a proposito con el `let _ =`.
        let _ = fs::remove_file(&self.csv_path);
        let _ = fs::remove_file(&self.vrt_path);
        let _ = fs::remove_file(&self.tif_path);
    }
}
 
fn export_points_to_csv(
    points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    height: usize,
    csv_path: &str,
) -> std::io::Result<()> {
    let mut content = String::from("x,y,z\n");
 
    for &(x, y) in points {
        let z = matrix[y][x];
        let y_inverted = (height - 1) - y;
        content.push_str(&format!("{x},{y_inverted},{z}\n"));
    }
 
    fs::write(csv_path, content)
}
 
// VRT minimo para que GDAL lea el CSV como puntos XYZ. El nombre de capa
// tiene que matchear el nombre del CSV sin extension, o gdal_grid no
// encuentra los puntos (falla con "0 features" en silencio).
fn write_vrt(vrt_path: &str, csv_path: &str, layer_name: &str) -> std::io::Result<()> {
    let csv_stem = std::path::Path::new(csv_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("sim_points");
 
    let content = format!(
        r#"<OGRVRTDataSource>
            <OGRVRTLayer name="{layer_name}">
                <SrcDataSource>{csv_path}</SrcDataSource>
                <SrcLayer>{csv_stem}</SrcLayer>
                <GeometryType>wkbPoint</GeometryType>
                <GeometryField encoding="PointFromColumns" x="x" y="y" z="z"/>
            </OGRVRTLayer>
        </OGRVRTDataSource>
        "#
    );
 
    fs::write(vrt_path, content)
}
 
/// Interpola usando gdal_grid como backend en vez de nuestras implementaciones
/// manuales de IDW/Kriging/TIN. Escribe los puntos a disco, llama al binario
/// de gdal_grid, y lee el resultado de vuelta.
pub fn interpolation_gdal_tin(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Result<Vec<Vec<f64>>, String> {
    // nombre unico por llamada, por las simulaciones concurrentes
    let tmp_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
 
    let tmp_dir = std::env::temp_dir();
    let csv_path = tmp_dir.join(format!("sim_points_{tmp_id}.csv"));
    let vrt_path = tmp_dir.join(format!("sim_points_{tmp_id}.vrt"));
    let tif_path = tmp_dir.join(format!("sim_output_{tmp_id}.tif"));
 
    let csv_path_str = csv_path.to_str().ok_or("Ruta de CSV inválida")?;
    let vrt_path_str = vrt_path.to_str().ok_or("Ruta de VRT inválida")?;
    let tif_path_str = tif_path.to_str().ok_or("Ruta de TIF inválida")?;

    // A partir de aca, pase lo que pase (exito o cualquiera de los `?` de
    // abajo cortando antes), este guard borra los 3 archivos al salir de
    // la funcion -- no hace falta acordarse de limpiar en cada punto de salida.
    let _cleanup = TempFilesGuard {
        csv_path: csv_path.clone(),
        vrt_path: vrt_path.clone(),
        tif_path: tif_path.clone(),
    };
 
    export_points_to_csv(measuring_points, matrix, geotiff.height, csv_path_str)
        .map_err(|e| format!("Error escribiendo CSV: {e}"))?;
 
    write_vrt(vrt_path_str, csv_path_str, "sim_points")
        .map_err(|e| format!("Error escribiendo VRT: {e}"))?;
 
    // -txe/-tye: extension en X/Y en coordenadas de pixel.
    // -outsize: mismo tamaño que el geotiff original.
    let output = Command::new("gdal_grid")
        .args([
            "-a", "linear",
            "-of", "GTiff",
            "-ot", "Float64",
            "-txe", "0", &geotiff.width.to_string(),
            "-tye", "0", &geotiff.height.to_string(),
            "-outsize", &geotiff.width.to_string(), &geotiff.height.to_string(),
            "-l", "sim_points",
            vrt_path_str,
            tif_path_str,
        ])
        .output()
        .map_err(|e| format!("No se pudo ejecutar gdal_grid (¿está instalado?): {e}"))?;
 
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gdal_grid falló: {stderr}"));
    }
 
    // gdal_grid devuelve el tif con geotransform "neutro" (origen 0,0, pixel
    // 1x1), asi que le copiamos el geotransform y la proyeccion reales del
    // geotiff original para que quede alineado igual que el resto.
    {
        let mut output_dataset = Dataset::open_ex(
            tif_path_str,
            DatasetOptions {
                open_flags: GdalOpenFlags::GDAL_OF_UPDATE | GdalOpenFlags::GDAL_OF_RASTER,
                ..DatasetOptions::default()
            },
        )
        .map_err(|e| format!("No se pudo abrir el el tiff temporal de gdal_grid para agregar la projeccion y geo_transform: {e}"))?;
 
        output_dataset
            .set_geo_transform(&geotiff.geo_transform)
            .map_err(|e| format!("Error seteando geo_transform al tiff temporal:: {e}"))?;
 
        if !geotiff.projection.is_empty() {
            output_dataset
                .set_projection(&geotiff.projection)
                .map_err(|e| format!("Error seteando projection al tiff temporal: {e}"))?;
        }
    }
 
    let result_matrix_struct = processing_geotiff(tif_path_str)
        .map_err(|e| format!("Error leyendo resultado de la tiff temporal al interpolar con gdal_grid: {e}"))?;
 
 
    // gdal_grid interpola tambien donde el geotiff original es no_data
    // (fuera del cuerpo de agua). Restauramos el no_data ahi para no mostrar
    // profundidades inventadas fuera del area real.
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = result_matrix_struct.data;
    
    #[allow(clippy::needless_range_loop)]
    for j in 0..geotiff.height {
        for i in 0..geotiff.width {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
            }
        }
    }
 
    Ok(result)
}