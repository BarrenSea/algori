use std::ops::{Add, Sub, Mul};

///离散傅立叶变换

use crate::structure::Complex;
pub async fn dft(signal: &[f64]) -> Vec<Complex<f64>>

{
    let n = signal.len();
    let mut spectrum = vec![Complex::new(f64::default(), f64::default()); n];

    for k in 0..n {
        let mut sum = Complex::new(f64::default(), f64::default());
        for t in 0..n {
            let angle = -2.0 * std::f64::consts::PI * k as f64  * t as f64 / n as f64;
            let c = Complex::new(angle.cos(), angle.sin());
            let value = Complex::new(signal[t], f64::default());
            sum = sum + value * c;
        }
        spectrum[k] = sum;
    }

    spectrum
}

// ///复数离散傅立叶变换
// use crate::structure::Complex;
// pub fn dft_complex(input: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
//     let n = input.len();
//     let mut output: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];

//     for k in 0..n {
//         let mut sum = Complex::new(0.0, 0.0);
//         for t in 0..n {
//             let exp = Complex::new(0.0, -2.0 * std::f64::consts::PI * (t as f64) * (k as f64) / (n as f64));
//             sum = input[t] * exp.exp() + sum;
//         }
//         output[k] = sum;
//     }

//     output
// }
