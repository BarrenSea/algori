use std::ops::{Add, Sub, Mul};

///离散傅立叶变换
/// # Examples
/// ```
///  let signal = vec![0.0, 1.0, 0.0, 1.0]; // Input signal
///	let spectrum = algori::math::dft(&signal);
///
///	for (k, value) in spectrum.iter().enumerate() {
///            println!("Frequency {}: {:?}", k, value);
///	}
///
/// ```


pub fn dft<T: Copy + Default + std::convert::From<f64>>(signal: &[T]) -> Vec<Complex<T>> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Into<f64>
{
    let n = signal.len();
    let mut spectrum = vec![Complex::new(T::default(), T::default()); n];

    for k in 0..n {
        let mut sum = Complex::new(T::default(), T::default());
        for t in 0..n {
            let angle = -2.0 * std::f64::consts::PI * k as f64 * t as f64 / n as f64;
            let c = Complex::new(angle.cos().into(), angle.sin().into());
            let value = Complex::new(signal[t], T::default());
            sum = sum + value * c;
        }
        spectrum[k] = sum;
    }

    spectrum
}

///复数离散傅立叶变换
use crate::structure::Complex;
pub fn dft_complex(input: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut output: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for t in 0..n {
            let exp = Complex::new(0.0, -2.0 * std::f64::consts::PI * (t as f64) * (k as f64) / (n as f64));
            sum = input[t] * exp.exp() + sum;
        }
        output[k] = sum;
    }

    output
}
