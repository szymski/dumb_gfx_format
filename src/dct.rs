use num::traits::Inv;

type T = f32;
const PI: T = std::f32::consts::PI;

pub fn dct(signal: &Vec<T>, k: u32) -> f32 {
    let n = signal.len() as u32;

    if k == 0 {
        let sum: T = signal.iter().sum();
        return sum * T::inv(T::sqrt((n as T)));
    }

    let mut sum: f32 = 0.0;
    for m in 0..n {
        sum += signal[m as usize] * (PI * k as T * (2.0 * m as T + 1.0) / (2.0 * n as T)).cos()
    }

    sum * (2.0 / (n as T)).sqrt()
}

pub fn dct_reverse(dct: &Vec<T>, size: u32, x: u32, y: u32) -> T {
    let dct_len = dct.len() as u32;
    let n = (size * size) as u32;

    let m: T = (y * size + x) as T;
    let mut sum: T = 0.0;

    for k in 1..dct_len {
        sum += dct[k as usize] * (PI * (k as T) * (2.0 * m + 1.0) / (2.0 * n as T)).cos();
    }

    1.0 / (n as T).sqrt() * dct[0] + (2.0 / (n as T)).sqrt() * sum
}
