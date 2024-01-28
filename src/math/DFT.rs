use std::ops::{Add, Sub, Mul};
use crate::structure::Complex;
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
