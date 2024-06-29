use super::Complex;
/// Discrete Fourier Transform (DFT)
/// # Examples
/// ```
/// use algori::math::{dft,Complex};
/// let input: Vec<f64> = vec!(0.0,1.0,2.0,3.0);
/// let result = dft(&input);
/// println!("{:?}",result);
///
/// ```
pub fn dft<T: Clone>(input: &[T]) -> Vec<Complex<f64>>
where
    f64: From<T>,
{
    let n = input.len();
    let mut output = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (t, x) in input.iter().enumerate() {
            // 2 pi / N * n * k
            let angle = -2.0 * core::f64::consts::PI * (t as f64) * (k as f64) / (n as f64);
            // 欧拉变换 e ^ -i * angle => cos(angle) + sin(angle)i
            let c = Complex::new(angle.cos(), angle.sin()) * Complex::new(x.clone().into(), 0.0f64);
            // X[n] * cos(angle) + sin(angle)i
            sum = c + sum;
        }
        output.push(sum);
    }
    output
}
