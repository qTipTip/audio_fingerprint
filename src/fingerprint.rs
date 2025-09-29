use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::{fft::SpectrogramConfig, peaks::Peak};

// Max 2 seconds diff between target and anchor when creating
// fingerprints
const MAX_TIME_DELTA_MS: u32 = 2000;
const NUM_TARGET_PEAKS: usize = 5;

// We define a fingerprint as a relationship between two peaks
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Fingerprint {
    pub freq1: u32,
    pub freq2: u32,
    pub time_delta: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SongMetaData {
    pub song_id: u32,
    pub title: String,
}
// The fingerprint database maps a fingerprint to where it was found (song_id, time_offset)
#[derive(Serialize, Deserialize)]
pub struct FingerprintDB {
    pub database: HashMap<Fingerprint, Vec<(u32, u32)>>,
    pub songs: HashMap<u32, SongMetaData>,
    pub total_fingerprints: usize,
}

impl FingerprintDB {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
            songs: HashMap::new(),
            total_fingerprints: 0,
        }
    }

    pub fn add_song(&mut self, metadata: SongMetaData, peaks: &[Peak], config: &SpectrogramConfig) {
        log::info!(
            "Adding song: {} with title: {}",
            metadata.song_id,
            metadata.title
        );
        let fingerprints = generate_fingerprints(peaks, &config);
        for (fingerprint, time_offset) in fingerprints {
            self.database
                .entry(fingerprint)
                .or_insert_with(Vec::new)
                .push((metadata.song_id, time_offset))
        }

        self.songs.insert(metadata.song_id, metadata);
        self.total_fingerprints += self.database.len();
    }

    pub fn recognize_song(
        &self,
        peaks: &[Peak],
        config: &SpectrogramConfig,
    ) -> Option<MatchResult> {
        log::info!("Recognizing song");

        let query_fingerprints = generate_fingerprints(peaks, config);
        let total_query_fingerprints = query_fingerprints.len();
        // Map (song_id, alignment_offset) to counter
        let mut vote_counter: HashMap<(u32, u32), u32> = HashMap::new(); // (song_id, alignment_offset)

        for (query_fingerprint, time_offset) in query_fingerprints {
            let fingerprint_match = match self.database.get(&query_fingerprint) {
                Some(fingerprint) => fingerprint,
                None => continue,
            };

            for (song_id, offset) in fingerprint_match {
                let alignment_offset = offset - time_offset; // Database time - query time
                *vote_counter
                    .entry((*song_id, alignment_offset))
                    .or_insert(0) += 1
            }
        }

        // Fetch the key corresponding to the highest number of votes
        let result = vote_counter.iter().max_by_key(|(_key, value)| **value);

        match result {
            Some(((song_id, offset), votes)) => {
                let confidence = *votes as f32 / total_query_fingerprints as f32;
                return Some(MatchResult::new(*song_id, confidence, *offset, *votes));
            }
            None => None,
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        log::info!(
            "Saving fingerprint database with {} songs and {} fingerprints",
            self.songs.len(),
            self.total_fingerprints
        );

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let bincode_config = bincode::config::standard();
        bincode::serde::encode_into_std_write(self, &mut writer, bincode_config)?;

        Ok(())
    }
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        log::info!("Loading fingerprint database");

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let bincode_config = bincode::config::standard();
        let db = bincode::serde::decode_from_reader(reader, bincode_config)?;

        Ok(db)
    }

    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        match Self::load(&path) {
            Ok(db) => Ok(db),
            Err(_) => {
                log::info!("Database not found, creating new one");
                Ok(Self::new())
            }
        }
    }
}

pub(crate) fn generate_fingerprints(
    peaks: &[Peak],
    config: &SpectrogramConfig,
) -> Vec<(Fingerprint, u32)> {
    log::info!("Generating fingerprint");
    let mut fingerprints = Vec::new();
    let mut peak_indices: Vec<usize> = (0..peaks.len()).collect();
    peak_indices.sort_by_key(|&i| peaks[i].time_bin);

    for (i, &anchor_i) in peak_indices.iter().enumerate() {
        let anchor = &peaks[anchor_i];
        let mut valid_targets = Vec::new();

        // Collect all valid targets
        for &target_i in &peak_indices[i + 1..] {
            let target = &peaks[target_i];
            let time_diff_ms =
                ((target.time_seconds(config) - anchor.time_seconds(config)) * 1000.0) as u32;

            if time_diff_ms > MAX_TIME_DELTA_MS {
                break;
            }

            if time_diff_ms >= 50 {
                valid_targets.push((target_i, time_diff_ms));
            }
        }

        // Shuffle the valid targets, and take up to NUM_TARGET_PEAKS
        if !valid_targets.is_empty() {
            for j in 0..valid_targets.len() {
                let k = fastrand::usize(j..valid_targets.len());
                valid_targets.swap(j, k);
            }

            let num_to_take = NUM_TARGET_PEAKS.min(valid_targets.len());
            for &(target_i, _time_diff) in valid_targets.iter().take(num_to_take) {
                let target = &peaks[target_i];
                let fingerprint = create_fingerprint(anchor, target, config);
                let time_offset_ms = (anchor.time_seconds(config) * 1000.0) as u32;
                fingerprints.push((fingerprint, time_offset_ms));
            }
        }
    }

    log::info!("Done generating fingerprints");
    fingerprints
}

fn create_fingerprint(anchor: &Peak, target: &Peak, config: &SpectrogramConfig) -> Fingerprint {
    let freq1 = anchor.frequency_hz(config) as u32;
    let freq2 = target.frequency_hz(config) as u32;

    // Make sure the fingerprints always has the lowest frequency first.
    let (f1, f2) = if freq1 <= freq2 {
        (freq1, freq2)
    } else {
        (freq2, freq1)
    };

    Fingerprint {
        freq1: f1,
        freq2: f2,
        time_delta: ((target.time_seconds(config) - anchor.time_seconds(config)) * 1000.0) as u32,
    }
}

pub struct MatchResult {
    pub song_id: u32,
    pub confidence: f32,  // 0.0 to 1.0
    pub time_offset: u32, // Where in the original song (ms)
    pub votes: u32,       // Number of matching fingerprints
}

impl MatchResult {
    pub fn new(song_id: u32, confidence: f32, time_offset: u32, votes: u32) -> MatchResult {
        Self {
            song_id,
            confidence,
            time_offset,
            votes,
        }
    }
}
