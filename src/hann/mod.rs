//! This module provides functions for applying Hann tapers to signal samples.
//!
//! # Functions
//!
//! - [`apply_hanning_taper`]: Applies a Hann taper to the provided samples.

/// Applies a Hann taper to the provided signal samples.
///
/// # Parameters
/// - `samples`: The signal samples to apply the taper to.
/// - `taper_length`: The length of the taper in samples.

use std::f64::consts::PI;

fn hann(x: f64, length: usize) -> f64{
    0.5 * (1.0 - (2.0 * PI * x / length as f64).cos())
}

pub fn apply_hanning_fade_in(samples: &mut [f64], taper_size: usize) {
    let window_size = taper_size * 2;

    let window: Vec<f64> = (0..window_size)
        .map(|i| hann(i as f64, window_size - 1))
        .collect();

    for i in 0..taper_size {
        samples[i] *= window[i];
    }
}

pub fn apply_hanning_fade_out(samples: &mut [f64], taper_size: usize) {
    let window_size = taper_size * 2;

    let window: Vec<f64> = (0..window_size)
        .map(|i| hann(i as f64, window_size - 1))
        .collect();

    for i in 0..taper_size {
        samples[samples.len() - taper_size + i] *= window[taper_size + i];
    }
}

/// Use a Hann window to taper the edges (start and end) of the signal.
pub fn apply_hanning_taper(samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();
    let taper_size = taper_size.min(total_samples / 2);

    apply_hanning_fade_in(samples, taper_size);
    apply_hanning_fade_out(samples, taper_size);
}
