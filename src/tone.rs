use std::f64::consts::PI;

/// Generates 16-bit PCM tones.
pub struct Tone {
    /// The sample rate to use in hertz.
    sample_rate: u32,
    /// The duration of the tone in seconds.
    duration_in_seconds: u32,
    /// The starting frequency to use in hertz (left channel).
    base_frequency: f64,
    /// The frequency difference to use in hertz (right channel).
    diff_frequency: f64
}

impl Tone {
    pub fn new(sample_rate: u32, duration_in_seconds: u32, base_frequency: f64, diff_frequency: f64) -> Self {
        Tone {
            sample_rate,
            duration_in_seconds,
            base_frequency,
            diff_frequency
        }
    }

    // Returns a 16-bit PCM sine wave tone for a given frequency.
    pub fn generate_tone(&self, frequency: f64) -> Vec<f64> {
        let samples_per_cycle = (self.sample_rate as f64) / frequency;
        let total_samples = self.duration_in_seconds * self.sample_rate;
 
        (0..total_samples)
        .map(|s| s as f64)
        .map(|sample| {
            f64::sin(sample / (samples_per_cycle * 2.0 * PI))
        })
        .collect()
    }

    /// Returns two offset 16-bit PCM sine wave tones.
    pub fn generate_tones(&self) -> Vec<Vec<f64>> {
        return vec![
            self.generate_tone(self.base_frequency),
            self.generate_tone(self.base_frequency + self.diff_frequency)
        ]
    }
}