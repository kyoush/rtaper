//! > **Apply windowing and tapering to the signal**
use std::f64::consts::PI;
use std::process::exit;

pub struct TaperSpec {
    pub taper_type: String,
    pub taper_length: usize,
}

pub fn apply_linear_fade_in (samples: &mut [f64], taper_size: usize) {
    for i in 0..taper_size {
        let factor = i as f64 / taper_size as f64;
        samples[i] *= factor;
    }
}

pub fn apply_linear_fade_out (samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();

    for i in 0..taper_size {
        let factor = 1.0 - (taper_size - i) as f64 / taper_size as f64;
        samples[total_samples - 1 - i] *= factor;
    }
}

// pub fn apply_hanning_fade_in(samples: &mut [f64], length: usize) {
//     let n = length.min(samples.len());

//     let window: Vec<f64> = (0..n)
//         .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos()))
//         .collect();

//     for i in 0..(n / 2) {
//         samples[samples.len() - (n / 2) + i] *= window[i + n / 2];
//     }
// }

pub fn apply_hanning_fade_out(samples: &mut [f64], length: usize) {
    let n = length.min(samples.len());

    let window: Vec<f64> = (0..n)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos()))
        .collect();

    for i in 0..(n / 2) {
        samples[samples.len() - (n / 2) + i] *= window[i + n / 2];
    }
}

/// Use a Hann window to taper the edges (start and end) of the signal.
pub fn apply_hanning_taper(samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();
    let taper_size = taper_size.min(total_samples / 2);

    // apply_hanning_fade_in(samples, taper_size);
    apply_hanning_fade_out(samples, taper_size);
}

/// Apply linear tapering at the start and end of the signal.
pub fn apply_linear_taper(samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();
    let taper_size = taper_size.min(total_samples / 2);

    apply_linear_fade_in(samples, taper_size);
    apply_linear_fade_out(samples, taper_size);
}

pub fn apply_taper(samples: &mut [f64], spec: &TaperSpec) {
    match spec.taper_type.as_str() {
        "linear" => {apply_linear_taper(samples, spec.taper_length)},
        "hann" => {apply_hanning_taper(samples, spec.taper_length)},
        _ => {
            eprintln!("Error: unknown taper type");
            exit(1);
        }
    }
}
