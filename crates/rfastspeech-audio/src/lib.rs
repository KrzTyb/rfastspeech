use enum_as_inner::EnumAsInner;
use std::path::Path;

mod wav;

/// Errors that could occur while operating with audio data
#[derive(thiserror::Error, Debug, EnumAsInner)]
pub enum AudioError {
    /// IoError
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error during reading/writing audio file
    #[error("{0}")]
    ReadWriteError(String),

    /// Error during converting audio
    #[error("{0}")]
    ConvertError(&'static str),

    /// Audio format not supported
    #[error("{0}")]
    NotSupported(&'static str),

    /// Other error
    #[error("{0}")]
    Other(String),
}

type AudioResult<T> = std::result::Result<T, AudioError>;

/// Contains audio data in Float32 format (mono)
#[derive(Debug)]
pub struct AudioData {
    sample_rate: u32,
    samples: Vec<f32>,
}

impl AudioData {
    /// Non-mutable access to audio data
    pub fn samples(&self) -> &Vec<f32> {
        &self.samples
    }

    /// Returns the number of samples
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// The number of samples per second
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Read audio data from .wav file
    ///
    /// The audio format will be automatically converted to Float32
    /// Stereo will be converted into mono (mean from two channels)
    pub fn from_wav_file<P: AsRef<Path>>(path: P) -> AudioResult<Self> {
        let file = std::fs::File::open(path)?;
        let buf_reader = std::io::BufReader::new(file);
        let decoder = wav::WavDecoder::new(buf_reader)?;
        Ok(decoder.try_into()?)
    }

    /// Create new audio data
    ///
    /// Panics if sample rate == 0
    pub fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        if sample_rate == 0 {
            panic!("Sample rate cannot be 0!");
        }
        AudioData {
            sample_rate,
            samples: data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_SAMPLE: &str = "test_resources/sine.wav";

    #[test]
    fn new_audio_data() {
        let audio_data = AudioData::new(vec![0.5, -0.1, 1.0, -1.0], 8000);
        assert_eq!(audio_data.sample_rate(), 8000);
        assert_eq!(audio_data.len(), 4);
    }

    #[test]
    fn read_from_file() {
        let audio_data = AudioData::from_wav_file(TEST_SAMPLE).unwrap();

        assert_eq!(audio_data.sample_rate(), 44100);
        assert_eq!(audio_data.len(), 2205);
    }
}
