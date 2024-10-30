
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

/// Apply linear tapering at the start and end of the signal.
pub fn apply_linear_taper(samples: &mut [f64], taper_size: usize) {
    let total_samples = samples.len();
    let taper_size = taper_size.min(total_samples / 2);

    apply_linear_fade_in(samples, taper_size);
    apply_linear_fade_out(samples, taper_size);
}
