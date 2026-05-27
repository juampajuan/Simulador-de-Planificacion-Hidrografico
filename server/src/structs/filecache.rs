use std::time::{SystemTime, UNIX_EPOCH};
// Importamos la matriz real para que sea la misma que usa el resto del sistema
pub use simulations::structs::depth_matrix::DepthMatrix;

#[derive(Clone)]
pub struct CacheItem {
    pub id: String,
    pub matrix: DepthMatrix,
    pub last_path: Vec<(usize, usize)>,
}

pub struct FileCache {
    items: Vec<(CacheItem, u64)>,
    limit: usize    
}

impl FileCache {
    pub fn new(limit: usize) -> Self {
        Self { items: Vec::new(), limit }
    }

    fn get_now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    // Upsert: Si existe actualiza, si no, crea.
    pub fn update_path(&mut self, id: String, matrix: DepthMatrix, path: Vec<(usize, usize)>) {
        let now = self.get_now();

        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == id) {
            self.items[pos].0.matrix = matrix;
            self.items[pos].0.last_path = path;
            self.items[pos].1 = now;
        } else {
            if self.items.len() >= self.limit {
                self.remove_oldest();
            }
            self.items.push((CacheItem { id, matrix, last_path: path }, now));
        }
    }

    pub fn get(&mut self, id: &str) -> Option<&CacheItem> {
        let now = self.get_now();
        if let Some(pos) = self.items.iter().position(|(it, _)| it.id == id) {
            self.items[pos].1 = now;
            return Some(&self.items[pos].0);
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