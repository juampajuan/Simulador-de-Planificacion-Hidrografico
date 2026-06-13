pub mod config_loader;
use std::path::Path;
use std::fs;

pub fn create_dirs(path: &str) -> Option<()> {
    fs::create_dir_all(Path::new(path).join("geotiffs")).ok()?;
    fs::create_dir_all(Path::new(path).join("simulations")).ok()?;
    Some(())
}