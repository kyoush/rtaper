//! > **Apply windowing and tapering to the signal**
use std::error::Error;

pub struct TaperSpec {
    pub taper_type: String,
    pub taper_length: usize,
}

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
