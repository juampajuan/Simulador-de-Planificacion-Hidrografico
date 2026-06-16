use std::fs;
use std::process::Command;

use gdal::{Dataset, DatasetOptions, GdalOpenFlags};

use crate::structs::depth_matrix::DepthMatrix;
use crate::processing::geotiff::processing_geotiff;


/// Métodos de gdal_grid que podemos usar.
/// El string es el que se le pasa directo a `-a` en gdal_grid.
pub enum GdalGridMethod {
    /// IDW con suavizado. smoothing=0 es IDW puro (como
    /// interpolation_idw_kdtrees); smoothing>0 difumina las
    /// franjas del recorrido.
    InverseDistance { power: f64, smoothing: f64 },
    /// Interpolación lineal sobre triangulación de Delaunay
    /// (equivalente a interpolation_tin).
    Linear,
}

impl GdalGridMethod {
    fn to_arg(&self) -> String {
        match self {
            GdalGridMethod::InverseDistance { power, smoothing } => {
                format!("invdist:power={power}:smoothing={smoothing}")
            }
            GdalGridMethod::Linear => "linear".to_string(),
        }
    }
}

/// Exporta los puntos medidos a un CSV con columnas x,y,z.
///
/// x es la columna de píxel directamente. y necesita invertirse:
/// en nuestra matriz la fila 0 está arriba (y crece hacia abajo,
/// igual que el buffer del GeoTIFF), pero gdal_grid trata "y" como
/// una coordenada geográfica donde Y crece hacia ARRIBA. Sin esta
/// inversión, el raster resultante queda espejado verticalmente.
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

/// Escribe el archivo .vrt que le dice a GDAL cómo leer el CSV
/// como una capa de puntos (x, y, z).
///
/// GDAL espera que <SrcLayer> coincida con el nombre del CSV sin
/// extensión (es el nombre de la capa interna que el driver de CSV
/// le asigna al archivo). Si no se especifica, GDAL busca una capa
/// con el mismo nombre que <OGRVRTLayer> y falla con
/// "Failed to find layer 'X' on datasource", devolviendo 0 features.
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

/// Interpola usando gdal_grid como backend.
///
/// `measuring_points` y `matrix` son los puntos reducidos que ya
/// armó `interpolate()` (después de reduce_measuring_points), igual
/// que para IDW/Kriging/TIN.
///
/// Devuelve la matriz interpolada del mismo tamaño que el GeoTIFF
/// original, o `Err` si gdal_grid no está disponible o falla.
pub fn interpolation_gdal_grid(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
    method: GdalGridMethod,
) -> Result<Vec<Vec<f64>>, String> {
    // Directorio temporal para los archivos intermedios.
    // Usamos un nombre único por llamada para soportar ejecuciones
    // concurrentes (varios alumnos corriendo simulaciones a la vez).
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

    // 1. Exportar puntos a CSV
    export_points_to_csv(measuring_points, matrix, geotiff.height, csv_path_str)
        .map_err(|e| format!("Error escribiendo CSV: {e}"))?;

    // 2. Escribir el .vrt que describe la capa de puntos
    write_vrt(vrt_path_str, csv_path_str, "sim_points")
        .map_err(|e| format!("Error escribiendo VRT: {e}"))?;

    // 3. Llamar a gdal_grid
    //
    //    -txe / -tye: extensión en X/Y, en coordenadas de píxel.
    //    -outsize: tamaño de salida = tamaño del GeoTIFF original
    //    -l: nombre de la capa (definido en el .vrt)
    let output = Command::new("gdal_grid")
        .args([
            "-a", &method.to_arg(),
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

    // 4. gdal_grid generó el .tif en coordenadas de píxel "neutras"
    //    (origen 0,0 y tamaño de píxel 1x1, sin rotación). Le copiamos
    //    el geotransform y la proyección del GeoTIFF original para que
    //    quede orientado/alineado igual que él — sin esto, el resultado
    //    se ve "derecho" aunque el original esté rotado.
    {
        let mut output_dataset = Dataset::open_ex(
            tif_path_str,
            DatasetOptions {
                open_flags: GdalOpenFlags::GDAL_OF_UPDATE | GdalOpenFlags::GDAL_OF_RASTER,
                ..DatasetOptions::default()
            },
        )
        .map_err(|e| format!("No se pudo abrir el resultado de gdal_grid para editar: {e}"))?;

        output_dataset
            .set_geo_transform(&geotiff.geo_transform)
            .map_err(|e| format!("Error seteando geo_transform: {e}"))?;

        if !geotiff.projection.is_empty() {
            output_dataset
                .set_projection(&geotiff.projection)
                .map_err(|e| format!("Error seteando projection: {e}"))?;
        }
    }

    // 5. Leer el .tif resultado reutilizando el lector de GeoTIFF existente
    let result_matrix_struct = processing_geotiff(tif_path_str)
        .map_err(|e| format!("Error leyendo resultado de gdal_grid: {e}"))?;

    // 6. Limpiar archivos temporales (best-effort, no falla si no existen)
    let _ = fs::remove_file(&csv_path);
    let _ = fs::remove_file(&vrt_path);
    let _ = fs::remove_file(&tif_path);

    // 7. gdal_grid interpola libremente, incluso donde el GeoTIFF
    //    original es no_data (fuera del área del cuerpo de agua).
    //    Restauramos no_data ahí para que la imagen final no
    //    muestre profundidades inventadas fuera del área real.
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = result_matrix_struct.data;

    for j in 0..geotiff.height {
        for i in 0..geotiff.width {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
            }
        }
    }

    Ok(result)
}