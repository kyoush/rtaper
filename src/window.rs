use std::f64::consts::PI;

pub fn linear(x: i32, length: usize) -> f64 {
    if x < length as i32 / 2 {
        x as f64 * (2.0 / length as f64)
    }else {
        x as f64 * (-2.0 / length as f64) + 2.0
    }
}

pub fn hann(x: i32, length: usize) -> f64 {
    0.5 * (1.0 - (2.0 * PI * x as f64 / length as f64).cos())
}

pub fn cosine(x: i32, length: usize) -> f64 {
    (((x as f64 / length as f64) * 2.0 * PI + PI).cos() + 1.0) / 2.0
}

pub fn blackman(x: i32, length: usize) -> f64 {
    0.42 - 0.5 * ((2.0 * PI * x as f64) / length as f64).cos()
         + 0.08 * ((4.0 * PI * x as f64) / length as f64).cos()

}
