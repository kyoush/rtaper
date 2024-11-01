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


// @todo fn cos_win(x: f64, length: usize) -> f64 {}

use std::error::Error;
use crate::{WindowType, FadeType, TaperSpec};

pub fn is_taper_length_zero(spec: &TaperSpec) -> bool {
    spec.taper_length == 0
}

fn do_apply_taper_fade_in(samples: &mut [f64], f: fn(i32, usize) -> f64, taper_size: usize) {
    let window_size = taper_size * 2;
    let end_index = window_size - 1;

    let window: Vec<f64> = (0..window_size).map(|i| f(i as i32, end_index)).collect();

    for i in 0..taper_size {
        samples[i] *= window[i];
    }
}

fn do_apply_taper_fade_out(samples: &mut [f64], f: fn(i32, usize) -> f64, taper_size: usize) {
    let window_size = taper_size * 2;
    let end_index = window_size - 1;

    let window: Vec<f64> = (0..window_size).map(|i| f(i as i32, end_index)).collect();

    for i in 0..taper_size {
        samples[samples.len() - taper_size + i] *= window[taper_size + i];
    }
}

pub fn do_taper(
    samples: &mut [f64],
    spec: &TaperSpec,
    fade_type: FadeType,
) -> Result<(), Box<dyn Error>> {
    let total_samples = samples.len();
    let taper_size = spec.taper_length.min(total_samples / 2);

    let window_func: fn(i32, usize) -> f64 = match spec.taper_type {
        WindowType::Linear => super::window::linear,
        WindowType::Hann   => super::window::hann,
        WindowType::Cosine => super::window::cosine,
        WindowType::Blackman => super::window::blackman,
    };

    match fade_type {
        FadeType::FadeIn  => { do_apply_taper_fade_in( samples, window_func, taper_size) },
        FadeType::FadeOut => { do_apply_taper_fade_out(samples, window_func, taper_size) },
    }

    Ok(())
}
