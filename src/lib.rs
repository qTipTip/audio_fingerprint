use crate::fingerprint::{FingerprintDB, MatchResult, SongMetaData};

mod audio;
mod error;
mod fft;
mod fingerprint;
mod peaks;

pub fn get_database() -> Option<FingerprintDB> {
    match FingerprintDB::load_or_create("audio_fingerprint.db") {
        Ok(db) => {
            Some(db)
        },
        Err(_) =>None 
    }
}

pub fn save_database(db: &FingerprintDB) {
    db.save("audio_fingerprint.db")
        .expect("Unable to write to database");
}
pub fn analyze_song(db: &mut FingerprintDB, song_path: &str) {
    log::debug!("Adding {} to song database", song_path);

    let samples = audio::load_wav(song_path).expect("Unable to read wav file");
    let config = fft::SpectrogramConfig::default();
    let spectrogram = fft::compute_spectrogram(&samples, config);
    let peaks = peaks::extract_peaks(&spectrogram);

    let song_metadata = SongMetaData {
        song_id: db.songs.len() as u32,
        title: String::from(song_path),
    };

    db.add_song(song_metadata, &peaks, &spectrogram.config);

    let total_fingerprints: usize = db.database.values().map(|v| v.len()).sum();
    let unique_fingerprints: usize = db.database.len();

    log::debug!("Generated {} total fingerprints", total_fingerprints);
    log::debug!("Reduced to {} unique fingerprints", unique_fingerprints);
    log::debug!(
        "Average collisions per fingerprint: {:.2}",
        total_fingerprints as f32 / unique_fingerprints as f32
    );

}

pub fn recognize_song(db: &FingerprintDB, song_query_path: &str) -> Option<(SongMetaData, MatchResult)> {
    let samples = audio::load_wav(song_query_path).expect("Unable to read wav file");
    let config = fft::SpectrogramConfig::default();
    let spectrogram = fft::compute_spectrogram(&samples, config);
    let peaks = peaks::extract_peaks(&spectrogram);

    db.recognize_song(&peaks, &spectrogram.config)
}
