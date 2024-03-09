use crate::{Error, Result};
use std::path::Path;
use std::path::PathBuf;

pub mod io {
    use super::*;

    /// Read the path list from the directory
    pub fn read_dir_entries(path: &Path) -> Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(path)
            .map_err(|e| Error::from(e).add_path(path))?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .collect())
    }
}
