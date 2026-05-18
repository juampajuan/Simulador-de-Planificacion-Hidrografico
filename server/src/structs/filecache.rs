// use simulations::structs::depth_matrix::DepthMatrix;

use std::time::{SystemTime, UNIX_EPOCH};

// TODO: Comentar y reemplazar, cuando tenga id.
#[derive(Debug)]
pub struct DepthMatrix {
    pub id: i32,
}

pub struct FileCache {
    items: Vec<(DepthMatrix, u64)>,
    limit: usize    
}

impl FileCache {
    pub fn new(limit: i32) -> Self {
        Self {
            items: Vec::new(),
            limit: limit as usize
        }
    }

    pub fn add(&mut self, matrix: DepthMatrix) {

        // Puedo usar unwrap aca?
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for (item, date) in &mut self.items {
            if item.id == matrix.id {
                *date = now;
                return;
            }
        }

        if self.items.len() >= self.limit {
            if let Some((index, _)) = self.items
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, fecha))| *fecha)
            {
                self.items.remove(index);
            }
        }

        self.items.push((matrix, now));
    }

    pub fn get(&mut self, id: i32) -> Option<&DepthMatrix> {

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for (item, date) in &mut self.items {
            if item.id == id {
                *date = now;
                return Some(item);
            }
        }
        None
    }

}