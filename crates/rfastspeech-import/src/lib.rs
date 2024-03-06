use rfastspeech_core::{bail, util, Result};
use std::path::Path;

mod huggingface;

/// Imported model
#[derive(Debug)]
pub enum Model {
    /// HuggingFace model with config
    HuggingFace(huggingface::HFModel),
}

/// Import model from directory
/// Chooses available format automatically
pub fn import(path: &Path) -> Result<Model> {
    let model_files = util::io::read_dir_entries(path.as_ref())?;
    log::debug!("Model files: {:?}", model_files);

    // Prefer HuggingFace
    let result = huggingface::try_import(&model_files);
    log::trace!("HuggingFace import result: {:?}", result);
    if let Ok(hf_model) = result {
        return Ok(Model::HuggingFace(hf_model));
    }

    bail!("Supported model not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    const HF_DUMMY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/huggingface/dummy_model");

    #[test]
    fn import_simple() {
        // Directory without model
        let without_model_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(import(without_model_path).is_err());
        // HF dummy
        let hf_dummy_path = Path::new(HF_DUMMY_DIR);
        assert!(matches!(
            import(hf_dummy_path).unwrap(),
            Model::HuggingFace(_)
        ));
    }
}
