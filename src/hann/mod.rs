use std::f64::consts::PI;

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
