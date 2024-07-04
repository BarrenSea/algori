/// Leibniz series to approximate pi
/// ```
/// use algori::math::leibniz_pi;
/// let a: f64 = leibniz_pi(100000);
/// ```
pub fn leibniz_pi(count: usize) -> f64 {
    let mut pi = 0.0;
    let mut n = 1.0;
    let mut sign = 1;
    for _ in 0..count {
        pi += 1.0 / n * sign as f64;
        sign = -sign;
        n += 2.0;
        //	println!(" pi is {}, n is {}",pi,n);
    }
    return pi * 4.0;
}
