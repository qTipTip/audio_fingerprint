use rustfft::{FftPlanner, num_complex::Complex};

// Converts time-domain samples into a spectrogram using FFT. Given this info, we find frequency
// peaks.
#[derive(Copy, Clone)]
pub struct SpectrogramConfig {
    pub window_size: usize, // The FFT window size
    pub stride: usize,      // The stride we slide the window along with.
    pub sample_rate: f32,
}
impl SpectrogramConfig {
    pub(crate) fn default() -> SpectrogramConfig {
        Self {
            window_size: 1024,
            stride: 512,
            sample_rate: 48000.0,
        }
    }
}

pub struct Spectrogram {
    pub data: Vec<Vec<f32>>,
    pub config: SpectrogramConfig,
}

impl Spectrogram {
    pub(crate) fn new(data: Vec<Vec<f32>>, config: SpectrogramConfig) -> Spectrogram {
        Self {
            data: data,
            config: config,
        }
    }
}

pub fn compute_spectrogram(samples: &[f32], config: SpectrogramConfig) -> Spectrogram {
    // Note on FFT:
    //
    // An FFT on 1024 _real_ samples, you get 1024 complex numbers back.
    // Each output bin represents a frequency:
    // Bin 0: 0Hz
    // Bin 1: 1*sample_rate / 1024 Hz
    // Bin 2: 2*sample_rate / 1024 Hz
    // ...
    // Bin 512: 512*sample_rate / 1024 Hz = sample_rate / 2 Hz (Nyquist frequency). See the [Nyquist frequency](https://en.wikipedia.org/wiki/Nyquist_frequency)
    //
    // For real-valued input, like the wavs herein, FFT output has conjugate symmetry, meaning we
    // can discard the second half of the output (redundant conjugates). The first half is carrying
    // all the meaningful frequency information without redundancy.

    log::debug!(
        "Computing spectrogram with:\n window_size: {}\n window_stride: {}\n sample_rate: {}",
        config.window_size,
        config.stride,
        config.sample_rate
    );
    // 1. Split samples
    let num_windows = (samples.len() - config.window_size) / config.stride + 1;
    log::debug!("Using num_windows: {}", num_windows);
    // 2. Apply FFT

    log::debug! {"Running FFT"}
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(config.window_size);

    // This vec holds the spectrogram data from each sample window
    let mut spectrogram_data = Vec::new();

    for i in 0..num_windows {
        let start = i * config.stride;
        let window = &samples[start..start + config.window_size];

        let mut complex_samples: Vec<Complex<f32>> =
            window.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut complex_samples);

        // 3. Convert to magnitudes
        let magnitudes: Vec<f32> = complex_samples
            .iter()
            .take(config.window_size / 2)
            .map(|&c| c.norm())
            .collect();

        spectrogram_data.push(magnitudes);
    }
    log::debug!("Done running FFT");
    // 4. Store in spectrogram struct
    Spectrogram::new(spectrogram_data, config)
}
