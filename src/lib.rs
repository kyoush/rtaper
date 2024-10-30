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

/// Specifies the type and length of the taper to be applied.
pub struct TaperSpec {
    /// The type of taper to apply, e.g., "linear" or "hann".
    pub taper_type: String,
    /// The length of the taper in samples.
    pub taper_length: usize,
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
pub fn apply_taper(samples: &mut [f64], spec: &TaperSpec) -> Result<(), Box<dyn Error>> {
    match spec.taper_type.as_str() {
        "linear" => {linear::apply_linear_taper(samples, spec.taper_length)},
        "hann" => {hann::apply_hanning_taper(samples, spec.taper_length)},
        _ => {
            return Err("unknown taper type".into());
        }
    }

    Ok(())
}

pub mod linear;
pub mod hann;
pub mod cos;
