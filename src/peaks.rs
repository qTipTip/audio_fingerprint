use crate::fft::{Spectrogram, SpectrogramConfig};

const MAGNITUDE_THRESHOLD: f32 = 10.0;
// A peak represents a prominent point in the 2D time-frequency grid we compute using
// compute_spectrogram.
// A Peak is made up of a reference to which point in time, and which part of the frequency
// spectrum corresponds to the peak.
#[derive(Debug, Clone)]
pub struct Peak {
    pub time_bin: usize,
    pub freq_bin: usize,
    pub magnitude: f32,
}

impl Peak {
    pub fn new(time_bin: usize, freq_bin: usize, magnitude: f32) -> Self {
        Self {
            time_bin,
            freq_bin,
            magnitude,
        }
    }

    pub fn frequency_hz(&self, config: &SpectrogramConfig) -> f32 {
        // Each frequency bin contains `sample_rate / window_size` number of herz. E.g., 44100 /
        // 1024 = 43 Hz per bin. So the frequency bin with index `freq_bin` corresponds to the
        // frequency `freq_bin * sample_rate / window_size`.
        self.freq_bin as f32 * config.sample_rate / config.window_size as f32
    }

    pub fn time_seconds(&self, config: &SpectrogramConfig) -> f32 {
        // Each time bin corresponds to `stride / sample_rate` number of seconds. E.g.,  512 /
        // 44100 = ~1.74s per time bin.
        (self.time_bin * config.stride) as f32 / config.sample_rate
    }
}

pub fn extract_peaks(spectrogram: &Spectrogram) -> Vec<Peak> {
    log::debug!("Extracting peaks");
    let mut all_peaks = Vec::<Peak>::new();

    // We look for peaks in a 16x16 window in the time-frequency grid.
    let time_window = 3;
    let freq_window = 3;
    // We iterate over each time-slice in the time-frequency grid, and compute peaks in each
    // window.

    for t in time_window..spectrogram.data.len() - time_window {
        for f in freq_window..spectrogram.data[t].len() - freq_window {
            let center = spectrogram.data[t][f];
            let mut is_peak = true;

            'peak_loop: for dt in -(time_window as i8)..time_window as i8 {
                for df in -(freq_window as i8)..freq_window as i8 {
                    // We skip checking the center itself.
                    if dt == 0 && df == 0 {
                        continue;
                    }

                    let t_idx = (t as i32 + dt as i32) as usize;
                    let f_idx = (f as i32 + df as i32) as usize;

                    // If the neighboring data point is larger, then the center is not a peak, so
                    // we break the inner loop.
                    if spectrogram.data[t_idx][f_idx] >= center {
                        is_peak = false;
                        break 'peak_loop;
                    }
                }
            }
            if is_peak && center > MAGNITUDE_THRESHOLD {
                all_peaks.push(Peak::new(t, f, center));
            }
        }
    }

    log::debug!("Extracted {} peaks", all_peaks.len());
    all_peaks
}

#[cfg(test)]
mod test {
    use crate::{fft::SpectrogramConfig, peaks::Peak};

    #[test]
    fn peak_conversion() {
        let p = Peak {
            time_bin: 150,
            freq_bin: 46,
            magnitude: 10.0,
        };

        let c = SpectrogramConfig::default();

        let h = p.frequency_hz(&c);
        let t = p.time_seconds(&c);
        assert_eq!(h, 1981.0546875);
        assert_eq!(t, 1.74149659864);
    }
}
