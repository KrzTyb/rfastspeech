use crate::{AudioData, AudioError, AudioResult};
use hound::{SampleFormat, WavReader};

fn i8_to_f32(value: i8) -> f32 {
    value as f32 / 128.0
}

fn i16_to_f32(value: i16) -> f32 {
    value as f32 / 32_768.0
}

fn i24_to_f32(value: i32) -> f32 {
    value as f32 / 8_388_608.0
}

fn i32_to_f32(value: i32) -> f32 {
    value as f32 / 2_147_483_648.0
}

pub struct WavDecoder<R>
where
    R: std::io::Read,
{
    reader: WavReader<R>,
}

impl<R> WavDecoder<R>
where
    R: std::io::Read,
{
    pub fn new(reader: R) -> AudioResult<Self> {
        let reader =
            WavReader::new(reader).map_err(|err| AudioError::ReadWriteError(err.to_string()))?;

        if reader.spec().channels > 2 {
            return Err(AudioError::NotSupported("Audio should be mono or stereo"));
        }

        Ok(Self { reader })
    }
}

impl<R: std::io::Read> TryInto<AudioData> for WavDecoder<R> {
    type Error = AudioError;

    fn try_into(self) -> Result<AudioData, Self::Error> {
        let spec = self.reader.spec();

        // Convert samples into pcmf32
        let mut samples: Vec<f32> = match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Int, 8) => self
                .reader
                .into_samples()
                .filter_map(Result::ok)
                .map(i8_to_f32)
                .collect(),
            (SampleFormat::Int, 16) => self
                .reader
                .into_samples()
                .filter_map(Result::ok)
                .map(i16_to_f32)
                .collect(),
            (SampleFormat::Int, 24) => self
                .reader
                .into_samples()
                .filter_map(Result::ok)
                .map(i24_to_f32)
                .collect(),
            (SampleFormat::Int, 32) => self
                .reader
                .into_samples()
                .filter_map(Result::ok)
                .map(i32_to_f32)
                .collect(),
            (SampleFormat::Float, 32) => {
                self.reader.into_samples().filter_map(Result::ok).collect()
            }
            (_sample_format, _bits_per_sample) => {
                return Err(AudioError::ReadWriteError(
                    "Unimplemented sample format".into(),
                ));
            }
        };

        // Convert stereo to mono by calculating the mean from two channels
        if spec.channels == 2 {
            samples = samples
                .chunks_exact(2)
                .map(|chunk| chunk.iter().sum::<f32>() / 2.0)
                .collect();
        }

        Ok(AudioData {
            sample_rate: spec.sample_rate,
            samples,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hound::{WavSpec, WavWriter};
    use std::io::Cursor;

    #[test]
    fn read_int8() {
        let input_samples = vec![-11_i8, 50, 0, 127];
        let mut file_buffer = Cursor::new(Vec::new());

        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 8,
                sample_format: SampleFormat::Int,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();
        for (data, expected) in audio_data.samples().iter().zip(input_samples) {
            assert_eq!(*data, i8_to_f32(expected));
        }
    }

    #[test]
    fn read_int16() {
        let input_samples = vec![i16::MIN, i16::MAX, -774, 8000];
        let mut file_buffer = Cursor::new(Vec::new());

        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();
        for (data, expected) in audio_data.samples().iter().zip(input_samples) {
            assert_eq!(*data, i16_to_f32(expected));
        }
    }

    #[test]
    fn read_int24() {
        let input_samples = vec![-663644, 12412, 0, 1];
        let mut file_buffer = Cursor::new(Vec::new());

        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 24,
                sample_format: SampleFormat::Int,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();
        for (data, expected) in audio_data.samples().iter().zip(input_samples) {
            assert_eq!(*data, i24_to_f32(expected));
        }
    }

    #[test]
    fn read_int32() {
        let input_samples = vec![-963644552_i32, 963644552, -1, 1];
        let mut file_buffer = Cursor::new(Vec::new());

        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 32,
                sample_format: SampleFormat::Int,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();
        for (data, expected) in audio_data.samples().iter().zip(input_samples) {
            assert_eq!(*data, i32_to_f32(expected));
        }
    }

    #[test]
    fn read_float() {
        let input_samples = vec![0.64_f32, 0.16674, -0.11];
        let mut file_buffer = Cursor::new(Vec::new());

        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 1,
                sample_rate: 8000,
                bits_per_sample: 32,
                sample_format: SampleFormat::Float,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();
        for (data, expected) in audio_data.samples().iter().zip(input_samples) {
            assert_eq!(*data, expected);
        }
    }

    #[test]
    fn incorrect_file() {
        let dummy_file = Cursor::new(vec![5_u8, 1, 7, 2, 3]);
        let result = WavDecoder::new(dummy_file);
        assert!(result.is_err());
    }

    #[test]
    fn read_stereo() {
        let input_samples = vec![0.64_f32, 0.16674];

        let mut file_buffer = Cursor::new(Vec::new());
        let mut wav_writer = WavWriter::new(
            &mut file_buffer,
            WavSpec {
                channels: 2,
                sample_rate: 8000,
                bits_per_sample: 32,
                sample_format: SampleFormat::Float,
            },
        )
        .unwrap();
        for sample in input_samples.iter() {
            wav_writer.write_sample(*sample).unwrap();
        }
        wav_writer.finalize().unwrap();
        file_buffer.set_position(0);

        let audio_data: AudioData = WavDecoder::new(file_buffer).unwrap().try_into().unwrap();

        // Expected converted stereo into mono (mean from two channels)
        assert_eq!(
            audio_data.samples(),
            &vec![input_samples.iter().sum::<f32>() / 2.0]
        );
    }
}
