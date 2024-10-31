//! This library provides various tapering functions for signal processing.
//!
//! The main functionality includes different tapering techniques such as linear, Hann, and cosine tapers.
//!
//! # Modules
//!
//! - [`linear`]: Provides linear tapering functions.
//! - [`hann`]: Provides Hann tapering functions.
//! - [`cos`]: Provides cosine tapering functions.

use std::error::Error;

mod taper;
mod window;

pub enum FadeType {
    FadeIn,
    FadeOut,
}

pub enum WindowType {
    Linear,
    Hann,
    Cosine,
}

/// Specifies the type and length of the taper to be applied.
pub struct TaperSpec {
    /// The type of taper to apply, e.g., "linear" or "hann".
    pub taper_type: WindowType,
    /// The length of the taper in samples.
    pub taper_length: usize,
}

pub fn apply_taper_fade_in(samples: &mut[f64], spec: &TaperSpec) -> Result<(), Box<dyn Error>> {
    taper::do_taper(samples, &spec, FadeType::FadeIn)?;
    Ok(())
}

pub fn apply_taper_fade_out(samples: &mut[f64], spec: &TaperSpec) -> Result<(), Box<dyn Error>> {
    taper::do_taper(samples, &spec, FadeType::FadeOut)?;
    Ok(())
}

/// Applies the specified taper to the given signal samples.
/// 
/// # Parameters
/// - `samples`: The signal samples to which the taper is applied.
/// - `spec`: The specification of the taper to apply, including type and length.
///
/// # Returns
/// - `Ok(())` if the taper is applied successfully.
/// - `Err(Box<dyn Error>)` if an invalid taper type is provided.
///
/// # Examples
/// ```
/// let mut samples = vec![0.0, 0.1, 0.3, 0.7, 1.0, 0.7, 0.3, 0.1, 0.0];
/// let spec = TaperSpec {
///     taper_type: "linear".to_string(),
///     taper_length: samples.len(),
/// };
/// apply_taper(&mut samples, &spec).unwrap();
/// ```
pub fn apply_taper_both(samples: &mut [f64], spec: &TaperSpec) -> Result<(), Box<dyn Error>> {
    apply_taper_fade_in(samples, spec)?;
    apply_taper_fade_out(samples, spec)?;
    Ok(())
}
