use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{db::{engine::DBEngine, queries_interface::student_simulations::get_all_simulation_images_locked}, structs::settings::Settings};

pub fn clean_unused_images(
    db: &Arc<Mutex<DBEngine>>,
    settings: &Settings,
) -> Result<(), Box<dyn Error>> {
    let images: HashSet<String> = get_all_simulation_images_locked(db)?;

    let images_dir = PathBuf::from(&settings.storage_path).join("images");

    for entry in fs::read_dir(&images_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }

        let Some(filename) = path.file_name().and_then(|f| f.to_str()) else {
            continue;
        };

        if !images.contains(filename) {
            // TODO: Logear como debug.
            println!("Deleting unused image: {}", filename);
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}