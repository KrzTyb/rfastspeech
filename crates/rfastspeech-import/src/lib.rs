use anyhow::Result;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[path = "safetensors.rs"]
mod safetensors_utils;
use safetensors_utils::SafeTensorsHandler;

pub fn import(path: &Path) -> Result<()> {
    let mut safetensors_path = PathBuf::new();
    let dirs = std::fs::read_dir(path)?;
    for dir in dirs {
        let path = dir?.path();
        let extension = path.extension().and_then(OsStr::to_str);
        if let Some(extension) = &extension {
            if *extension == "safetensors" {
                safetensors_path = path;
                break;
            }
        }
    }

    let _model_params = unsafe { SafeTensorsHandler::load(&safetensors_path) }?;

    anyhow::bail!("Not yet implemented")
}
