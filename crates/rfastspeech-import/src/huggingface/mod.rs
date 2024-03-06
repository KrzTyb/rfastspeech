use rfastspeech_core::{bail, Error, Result};
use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

#[path = "safetensors.rs"]
mod safetensors_utils;
use safetensors_utils::SafeTensorsHandler;

#[derive(Debug)]
/// Represents imported model config
pub struct HFConfig {
    pub model_type: String,
    _other: serde_json::Value,
}

#[derive(Debug)]
/// Represents imported model
pub struct HFModel {
    pub config: HFConfig,
    variables: SafeTensorsHandler,
}

impl HFModel {
    /// Check if the model have a tensor with a specified name
    pub fn contains(&self, tensor_name: &str) -> bool {
        self.variables.contains(tensor_name)
    }
}

/// Try import HuggingFace model
pub fn try_import(entries: &Vec<PathBuf>) -> Result<HFModel> {
    let mut config_path = PathBuf::new();
    let mut safetensors_path = PathBuf::new();

    // Read files
    for entry in entries {
        if entry.is_file() {
            // config.json
            if let Some(file_name) = entry.file_name().and_then(OsStr::to_str) {
                if file_name == "config.json" {
                    config_path = entry.clone();
                }
            }

            // Model variables
            let extension = entry.extension().and_then(OsStr::to_str);
            if let Some(extension) = &extension {
                if *extension == "safetensors" {
                    safetensors_path = entry.clone();
                }
            }
        }
    }

    if !config_path.is_file() {
        bail!("config file not found");
    }
    if !safetensors_path.is_file() {
        bail!("safetensors file not found");
    }

    let model_config = load_config(config_path)?;
    let model_params = unsafe { SafeTensorsHandler::load(&safetensors_path) }?;

    Ok(HFModel {
        config: model_config,
        variables: model_params,
    })
}

fn load_config<P: AsRef<Path>>(path: P) -> Result<HFConfig> {
    let path = path.as_ref();
    let json_config: serde_json::Value;
    {
        let file = File::open(path)?;
        json_config =
            serde_json::from_reader(file).map_err(|err| Error::from(err).add_path(path))?;
    }

    let model_type = json_config
        .get("model_type")
        .and_then(|value| value.as_str())
        .ok_or(
            Error::Message("Config json doesn't contain model_type (str).".to_string())
                .add_path(path),
        )?
        .to_string();

    Ok(HFConfig {
        model_type,
        _other: json_config,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const MAIN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/huggingface");

    #[test]
    fn import_simple() {
        let dummy_path = Path::new(MAIN_DIR).join("dummy_model");

        let mut entries = Vec::new();

        // Empty
        assert!(try_import(&entries).is_err());

        // Only config file
        entries.push(dummy_path.join("config.json"));
        assert!(try_import(&entries).is_err());

        // All required files
        entries.push(dummy_path.join("model.safetensors"));

        let model = try_import(&entries).unwrap();
        assert_eq!(model.config.model_type, "dummy");
        assert!(model.contains("dense.weight"));
        assert!(model.contains("dense.bias"));
        assert!(model.contains("intermediate.weight"));
        assert!(model.contains("intermediate.bias"));
        assert!(model.contains("layer_norm.weight"));
        assert!(model.contains("layer_norm.bias"));
    }
}
