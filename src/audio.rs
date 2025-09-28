use hound::{SampleFormat, WavReader};
use log;

use crate::error::AudioError;

pub fn load_wav(path_to_audio: &str) -> Result<Vec<f32>, AudioError> {
    log::debug!("Loading wav from {path_to_audio}");

    let mut wav_reader = WavReader::open(path_to_audio)?;
    log::debug!("Wav duration in samples: {:?}", wav_reader.duration());

    let spec = wav_reader.spec();
    log::debug!("WavSpec sample_format: {:?}", spec.sample_format);
    log::debug!("WavSpec sample_rate: {:?}", spec.sample_rate);
    log::debug!("WavSpec channels: {:?}", spec.channels);
    log::debug!("WavSpec bits / sample: {:?}", spec.bits_per_sample);
    log::debug!(
        "Song duration: {:?}s",
        wav_reader.duration() as f32 / spec.sample_rate as f32
    );

    let samples = match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Int, 16) => wav_reader
            .samples::<i16>()
            .map(|x| x.map(|sample| sample as f32 / i16::MAX as f32))
            .collect::<Result<Vec<f32>, _>>()?,
        (SampleFormat::Int, 32) => wav_reader
            .samples::<i32>()
            .map(|x| x.map(|sample| sample as f32 / i32::MAX as f32))
            .collect::<Result<Vec<f32>, _>>()?,
        (SampleFormat::Int, 24) => wav_reader
            .samples::<i32>()
            .map(|x| x.map(|sample| sample as f32 / (1i32 << 23) as f32))
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

    // We convert to mono, should halve the work needed.
    let mono_samples: Vec<f32> = if spec.channels == 2 {
        samples
            .chunks(2)
            .map(|pair| (pair[0] + pair[1]) / 2.0)
            .collect()
    } else {
        samples
    };

    Ok(mono_samples)
}
