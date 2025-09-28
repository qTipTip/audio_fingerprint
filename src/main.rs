mod audio;
mod error;
mod fft;
mod fingerprint;
mod peaks;

use crate::{
    fft::SpectrogramConfig,
    fingerprint::{FingerprintDB, generate_fingerprints},
    peaks::extract_peaks,
};

fn main() {
    // Configure a simple logger
    simple_logger::SimpleLogger::new().init().unwrap();

    let samples =
        audio::load_wav(r"test_audio/03 Karesuando Camping.wav").expect("Unable to read wav file");
    let config = SpectrogramConfig::default();
    let spectrogram = fft::compute_spectrogram(&samples, config);
    let peaks = extract_peaks(&spectrogram);

    let mut db = FingerprintDB::new();
    db.add_song(1, &peaks, &config);

    let total_fingerprints: usize = db.database.values().map(|v| v.len()).sum();
    let unique_fingerprints: usize = db.database.len();

    log::debug!("Generated {} total fingerprints", total_fingerprints);
    log::debug!("Reduced to {} unique fingerprints", unique_fingerprints);
    log::debug!(
        "Average collisions per fingerprint: {:.2}",
        total_fingerprints as f32 / unique_fingerprints as f32
    );

    for (fingerprint, locations) in db.database.iter().take(10) {
        log::debug!(
            "Fingerprint: {}Hz->{}Hz, dt={}ms appears {}times",
            fingerprint.freq1,
            fingerprint.freq2,
            fingerprint.time_delta,
            locations.len(),
        );

        for (song_id, time_offset) in locations.iter().take(3) {
            log::debug!("\t -> Song {} at {}ms", song_id, time_offset);
        }
    }
}
