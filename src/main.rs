mod audio;
mod error;
mod fft;
mod fingerprint;
mod peaks;

use crate::{fft::SpectrogramConfig, fingerprint::generate_fingerprints, peaks::extract_peaks};

fn main() {
    // Configure a simple logger
    simple_logger::SimpleLogger::new().init().unwrap();

    let samples = audio::load_wav("test_audio/example.wav").expect("Unable to read wav file");
    let config = SpectrogramConfig::default();
    let spectrogram = fft::compute_spectrogram(&samples, config);
    let peaks = extract_peaks(&spectrogram);

    log::debug!("Found {} total peaks", peaks.len());

    for peak in peaks.iter().take(10) {
        log::debug!(
            "Peak: {:.3}Hz at {:.2}s (magnitude: {:.10})",
            peak.frequency_hz(&spectrogram.config),
            peak.time_seconds(&spectrogram.config),
            peak.magnitude
        );
    }

    let fingerprints = generate_fingerprints(&peaks, &config);
    log::debug!("Found fingerprints, {:?}", fingerprints.iter().take(10));
}
