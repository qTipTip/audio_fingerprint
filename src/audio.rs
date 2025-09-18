use hound::{SampleFormat, WavReader};
use log;

use crate::error::AudioError;

pub fn load_wav(path_to_audio: &str) -> Result<Vec<f32>, AudioError> {
    log::debug!("Loading wav from {path_to_audio}");

    let mut wav_reader = WavReader::open(path_to_audio)?;
    log::debug!("Wav duration in samples: {:?}", wav_reader.duration());

    let spec = wav_reader.spec();
    log::debug!("WavSpec sample_format: {:?}", spec.sample_format);
    log::debug!("WavSpec bits / sample: {:?}", spec.bits_per_sample);

    let samples = match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Int, 16) => wav_reader
            .samples::<i16>()
            .map(|x| x.map(|sample| sample as f32 / i16::MAX as f32))
            .collect::<Result<Vec<f32>, _>>()?,
        (SampleFormat::Int, 32) => wav_reader
            .samples::<i32>()
            .map(|x| x.map(|sample| sample as f32 / i32::MAX as f32))
            .collect::<Result<Vec<f32>, _>>()?,
        (SampleFormat::Float, 32) => wav_reader
            .samples::<f32>()
            .collect::<Result<Vec<f32>, _>>()?,
        _ => {
            log::debug!("Unable to handle the format");
            return Err(AudioError::UnsupportedFormat(
                spec.sample_format,
                spec.bits_per_sample,
            ));
        }
    };

    Ok(samples)
}
