use crate::fft::{Spectrogram, SpectrogramConfig};

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
    let mut all_peaks = Vec::<Peak>::new();

    // We iterate over each time-slice in the time-frequency grid, and compute peaks in each
    // window.
    for (time_bin, freq_magnitudes) in spectrogram.data.iter().enumerate() {
        let peaks_in_this_window = find_frequency_peaks(&freq_magnitudes, time_bin);
        all_peaks.extend(peaks_in_this_window);
    }

    all_peaks
}

fn find_frequency_peaks(magnitudes: &[f32], time_bin: usize) -> Vec<Peak> {
    // We use a very naive frequency peak algorithm. It will simply find ALL local maxima in the
    // frequency spectrum.
    //
    // TODO: Implement adaptive thresholding
    let mut peaks = Vec::<Peak>::new();

    let num_magnitudes = magnitudes.len();

    // Edge conditions: start
    if magnitudes[0] > magnitudes[1] {
        peaks.push(Peak {
            time_bin,
            freq_bin: 0,
            magnitude: magnitudes[0],
        });
    }

    for i in 1..num_magnitudes - 1 {
        if (magnitudes[i] > magnitudes[i - 1]) && (magnitudes[i] > magnitudes[i + 1]) {
            peaks.push(Peak {
                time_bin,
                freq_bin: i,
                magnitude: magnitudes[i],
            });
        }
    }
    // Edge conditions: end
    if magnitudes[num_magnitudes - 1] > magnitudes[num_magnitudes - 2] {
        peaks.push(Peak {
            time_bin,
            freq_bin: num_magnitudes - 1,
            magnitude: magnitudes[num_magnitudes - 1],
        });
    }

    peaks
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
