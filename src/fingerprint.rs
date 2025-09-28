use std::collections::HashMap;

use crate::{fft::SpectrogramConfig, peaks::Peak};

// Max 2 seconds diff between target and anchor when creating
// fingerprints
const MAX_TIME_DELTA_MS: u32 = 2000;
const NUM_TARGET_PEAKS: usize = 5;

// We define a fingerprint as a relationship between two peaks
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Fingerprint {
    pub freq1: u32,
    pub freq2: u32,
    pub time_delta: u32,
}

// The fingerprint database maps a fingerprint to where it was found (song_id, time_offset)
pub struct FingerprintDB {
    pub database: HashMap<Fingerprint, Vec<(u32, u32)>>,
}

impl FingerprintDB {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    pub fn add_song(&mut self, song_id: u32, peaks: &[Peak], config: &SpectrogramConfig) {
        log::debug!("Adding song: {song_id}");
        let fingerprints = generate_fingerprints(peaks, &config);

        for (fingerprint, time_offset) in fingerprints {
            self.database
                .entry(fingerprint)
                .or_insert_with(Vec::new)
                .push((song_id, time_offset))
        }
    }
}

pub(crate) fn generate_fingerprints(
    peaks: &[Peak],
    config: &SpectrogramConfig,
) -> Vec<(Fingerprint, u32)> {
    log::debug!("Generating fingerprint");
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

    log::debug!("Done generating fingerprints");
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
