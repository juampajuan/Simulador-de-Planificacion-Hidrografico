use common::PathParameters;
pub use simulations::structs::depth_matrix::DepthMatrix;
use std::{
    sync::mpsc::Sender,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::logging::{logger::debug_logger, structs::ThreadMessage};

/// Recorrido cacheado: las coordenadas calculadas junto con los parámetros con los que
/// se generaron, para poder reutilizarlo solo si el alumno vuelve a pedir lo mismo.
#[derive(Clone)]
pub struct PathData {
    pub coordinates: Vec<(usize, usize)>,
    pub parameters: PathParameters,
}

// CACHÉ DE MAPAS
pub struct MapCacheItem {
    pub geotiff_path: String,
    pub matrix: DepthMatrix,
}

// CACHÉ DE RECORRIDOS (y parámetros)
pub struct PathCacheItem {
    pub student_key: i64,
    pub last_path: PathData,
}

/// Caché de mapas y recorridos en memoria, con un límite configurable de elementos
/// para desalojar los más viejos en desuso. Se indexa por la ruta física del GeoTIFF para los mapas y por el ID del alumno para los recorridos y sus parámetros.
pub struct FileCache {
    // Listas emparejadas con su timestamp u64 para el algoritmo LRU
    maps: Vec<(MapCacheItem, u64)>,
    user_paths: Vec<(PathCacheItem, u64)>,
    pub limit: usize,
    tx: Sender<ThreadMessage>,
}

impl FileCache {
    pub fn new(limit: usize, tx: Sender<ThreadMessage>) -> Self {
        Self {
            maps: Vec::new(),
            user_paths: Vec::new(),
            limit,
            tx: tx,
        }
    }

    fn get_now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_else(|e| {
                eprintln!("Error al obtener el tiempo del sistema: {}", e);
                0
            })
    }

    /// Almacena una DepthMatrix indexándola estrictamente por su ruta de archivo.
    pub fn update_map(&mut self, geotiff_path: String, matrix: DepthMatrix) {
        let now = self.get_now();
        let debug_log = debug_logger(&self.tx, "CACHE");
        debug_log(&format!(
            "Se solicita actualizar o agregar un DepthMatrix (Path: {}).",
            &geotiff_path
        ));

        // Buscamos si el archivo ya fue cargado por otro alumno previamente
        if let Some(pos) = self
            .maps
            .iter()
            .position(|(it, _)| it.geotiff_path == geotiff_path)
        {
            debug_log(&format!(
                "Fue encontrado en la posición {}, actualizamos su fecha de acceso. (Path: {})",
                pos, &geotiff_path
            ));
            self.maps[pos].0.matrix = matrix;
            self.maps[pos].1 = now;
        } else {
            debug_log(&format!(
                "No se encontró en el cache, lo agregamos. (Path: {})",
                &geotiff_path
            ));
            // Si la caché de mapas se llenó, desalojamos el mapa más viejo en desuso
            if self.maps.len() >= self.limit
                && let Some((index, _)) = self.maps.iter().enumerate().min_by_key(|(_, (_, f))| *f)
            {
                debug_log(&format!(
                    "La cache (DepthMatrix) esta llena, eliminamos la entrada que lleva mas tiempo si ser accedida (Index: {}).",
                    index
                ));
                self.maps.remove(index);
            }
            self.maps.push((
                MapCacheItem {
                    geotiff_path,
                    matrix,
                },
                now,
            ));
        }
    }

    /// Obtiene la matriz compartida usando únicamente la ruta física del GeoTIFF.
    pub fn get_map(&mut self, geotiff_path: &str) -> Option<&DepthMatrix> {
        let now = self.get_now();
        let debug_log = debug_logger(&self.tx, "CACHE");
        debug_log(&format!(
            "Se solicita buscar el DepthMatrix (Path: {}).",
            &geotiff_path
        ));

        if let Some(pos) = self
            .maps
            .iter()
            .position(|(it, _)| it.geotiff_path == geotiff_path)
        {
            debug_log(&format!(
                "Se encontró la DepthMatrix (index: {}), actualizamos su fecha de acceso, antes de retornarlo. (Path: {})",
                pos, &geotiff_path
            ));
            self.maps[pos].1 = now; // Actualiza último acceso global
            return Some(&self.maps[pos].0.matrix);
        }
        debug_log(&format!("No fue encontrado. (Path: {})", &geotiff_path));
        None
    }

    /// Guarda o actualiza el recorrido específico de un alumno y sus parametros.
    pub fn update_path(
        &mut self,
        student_key: i64,
        coordinates: Vec<(usize, usize)>,
        params: PathParameters,
    ) {
        let now = self.get_now();
        let path_data = PathData {
            coordinates,
            parameters: params,
        };

        let debug_log = debug_logger(&self.tx, "CACHE");
        debug_log(&format!(
            "Se intenta agregar o actualizar un path para alumno (Id: {}) con parametros {:?}",
            student_key, &path_data.parameters
        ));

        if let Some(pos) = self
            .user_paths
            .iter()
            .position(|(it, _)| it.student_key == student_key)
        {
            debug_log(&format!(
                "Se encontró un path (index: {}) del alumno (Id: {}) lo reemplazamos con el nuevo, con parametros {:?}",
                pos, student_key, &path_data.parameters
            ));
            self.user_paths[pos].0.last_path = path_data;
            self.user_paths[pos].1 = now;
        } else {
            debug_log(&format!(
                "No se encontró un path en el cache, lo agregamos para alumno (Id: {}) con parametros {:?}",
                student_key, &path_data.parameters
            ));
            // Si se llena la caché de caminos, remueve el recorrido más viejo
            if self.user_paths.len() >= self.limit
                && let Some((index, _)) = self
                    .user_paths
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, (_, f))| *f)
            {
                debug_log(&format!(
                    "La cache (Path) esta llena, eliminamos la entrada que lleva mas tiempo si ser accedida (Index: {}).",
                    index
                ));
                self.user_paths.remove(index);
            }
            self.user_paths.push((
                PathCacheItem {
                    student_key,
                    last_path: path_data,
                },
                now,
            ));
        }
    }

    /// Devuelve el camino del alumno sólo si coincide con sus parámetros de simulación actuales.
    pub fn get_path_if_valid(
        &mut self,
        student_key: i64,
        current_params: &PathParameters,
    ) -> Option<Vec<(usize, usize)>> {
        let now = self.get_now();
        let debug_log = debug_logger(&self.tx, "CACHE");
        debug_log(&format!(
            "Se solicita buscar un path para alumno (Id: {}) con parametros {:?}",
            student_key, current_params
        ));

        if let Some(pos) = self
            .user_paths
            .iter()
            .position(|(it, _)| it.student_key == student_key)
        {
            self.user_paths[pos].1 = now;

            let path_data = &self.user_paths[pos].0.last_path;
            if path_data.parameters == *current_params {
                debug_log(&format!(
                    "Se encontró un path para alumno (Id: {}) en la posicion {} con parametros {:?}. Actualizamos fecha de acceso antes de retornar.",
                    student_key, pos, current_params
                ));
                return Some(path_data.coordinates.clone());
            }
        }
        debug_log(&format!(
            "No se encontró un path para alumno (Id: {}) con parametros {:?}",
            student_key, current_params
        ));
        None
    }
}
