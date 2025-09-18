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

    // We start by sorting the peaks by time, so it's easy to find peaks close in time.
    let mut peak_indices: Vec<usize> = (0..peaks.len()).collect();
    peak_indices.sort_by_key(|&i| peaks[i].time_bin);

    // For each peak, we look for N peaks in the future and create a fingerprint  for each
    for (i, &anchor_i) in peak_indices.iter().enumerate() {
        // We get a reference to the anchor peak, and initialize a counter for the number of target
        // peaks.
        let anchor = &peaks[anchor_i];
        let mut target_count = 0;

        for &target_i in &peak_indices[i + 1..] {
            // For each target, we compare the difference in time to the anchor, and discard the
            // target as a candidate if the time diff is too large.
            let target = &peaks[target_i];
            let time_diff_ms =
                ((target.time_seconds(config) - anchor.time_seconds(config)) * 1000.0) as u32;

            if time_diff_ms > MAX_TIME_DELTA_MS {
                log::trace!(
                    "Time diff {}ms > {}ms. Skipping",
                    time_diff_ms,
                    MAX_TIME_DELTA_MS
                );
                break;
            }

            let fingerprint = create_fingerprint(anchor, target, config);
            let time_offset_ms = (anchor.time_seconds(config) * 1000.0) as u32;
            fingerprints.push((fingerprint, time_offset_ms));

            target_count += 1;
            if target_count >= NUM_TARGET_PEAKS {
                break;
            }
        }
    }

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
