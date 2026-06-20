use std::time::{SystemTime, UNIX_EPOCH};
pub use simulations::structs::depth_matrix::DepthMatrix;
use common::PathParameters;

#[derive(Clone)]
pub struct PathData {
    pub coordinates: Vec<(usize, usize)>,
    pub parameters: PathParameters,
}

#[derive(Clone)]
pub struct CacheItem {
    pub id: String,
    pub geotiff_path: String,
    pub matrix: DepthMatrix,
    pub last_path: Option<PathData>, 
}

pub struct FileCache {
    items: Vec<(CacheItem, u64)>,
    pub limit: usize,
}

impl FileCache {
    pub fn new(limit: usize) -> Self {
        Self { items: Vec::new(), limit }
    }

    fn get_now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn update_map(&mut self, cache_key: String, geotiff_path: String, matrix: DepthMatrix) {
        let now = self.get_now();

        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == cache_key) {
            self.items[pos].0.geotiff_path = geotiff_path;
            self.items[pos].0.matrix = matrix;
            self.items[pos].1 = now;
        } else {
            if self.items.len() >= self.limit {
                self.remove_oldest();
            }
            self.items.push((
                CacheItem { 
                    id: cache_key, 
                    geotiff_path, 
                    matrix, 
                    last_path: None 
                }, 
                now
            ));
        }
    }

    pub fn get_map(&mut self, cache_key: &str, geotiff_path: &str) -> Option<&DepthMatrix> {
        let now = self.get_now();
        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == cache_key) 
            && self.items[pos].0.geotiff_path == geotiff_path {
                self.items[pos].1 = now;
                return Some(&self.items[pos].0.matrix);
            }
        
        None
    }

    pub fn update_path(&mut self, cache_key: String, coordinates: Vec<(usize, usize)>, params: PathParameters) {
        let now = self.get_now();

        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == cache_key) {
            self.items[pos].0.last_path = Some(PathData { coordinates, parameters: params });
            self.items[pos].1 = now;
        }
    }

    pub fn get_path_if_valid(&mut self, cache_key: &str, current_params: &PathParameters) -> Option<Vec<(usize, usize)>> {
        let now = self.get_now();
        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == cache_key) {
            self.items[pos].1 = now;
            
            if let Some(ref path_data) = self.items[pos].0.last_path 
                && path_data.parameters == *current_params {
                    return Some(path_data.coordinates.clone());
                }
        }
        
        None
    }

    fn remove_oldest(&mut self) {
        if let Some((index, _)) = self.items.iter().enumerate()
            .min_by_key(|(_, (_, fecha))| *fecha) {
            self.items.remove(index);
        }
    }
}