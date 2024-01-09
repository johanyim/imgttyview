use crate::utils::error;
use std::path::{Path, PathBuf};

pub fn init_open(path: &Path) -> error::Result<(&Path, PathBuf)> {

    if path.is_file() {
        let file = path;
        let directory = path.parent()
            .expect("file should have parent");
        return Ok((directory, file.to_owned()));
    }

    //default
    let mut directory = Path::new("./");
    if path.is_dir() {
        directory = path;
    }

    //first in directory
    if let Ok(mut entries) = directory.read_dir() {
        if let Some(entry) = entries
            .find_map(|e| e.ok()) {
            let file_path = entry.path();

            return Ok((directory, file_path));
        }
    }

    return Err(error::Error::FileNotFound("Not found".to_owned()));
}
