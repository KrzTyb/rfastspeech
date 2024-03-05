use anyhow::Result;
use memmap2::{Mmap, MmapOptions};
use safetensors::SafeTensors;
use std::{fs::File, path::Path};

#[derive(yoke::Yokeable)]
struct _SafeTensors<'a>(SafeTensors<'a>);

pub struct SafeTensorsHandler {
    _tensors: yoke::Yoke<_SafeTensors<'static>, Mmap>,
}

impl SafeTensorsHandler {
    /// Load safetensors with memory mapped file and wrap them into handler struct
    ///
    /// # Safety
    ///
    /// The unsafe is inherited from [`memmap2::MmapOptions`].
    pub unsafe fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let buffer = MmapOptions::new().map(&file)?;

        let safetensors = yoke::Yoke::<_SafeTensors<'static>, Mmap>::try_attach_to_cart(
            buffer,
            |data: &[u8]| {
                let st = safetensors::SafeTensors::deserialize(data)?;
                Ok::<_, anyhow::Error>(_SafeTensors(st))
            },
        )?;

        Ok(Self {
            _tensors: safetensors,
        })
    }
}