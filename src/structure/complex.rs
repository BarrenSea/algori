
use std::cmp::Ordering;


use std::ops::{Add, Sub, Mul};

use std::ops::Div;

#[derive(Debug, Copy, Clone)]
///复数

pub struct Complex<T> {
    pub real: T,
    pub imag: T,
}

///是否相等 == !=实现
impl<T> PartialEq for Complex<T>
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl<T> Eq for Complex<T> where T: Eq {}

///复数比大小实现
impl<T> PartialOrd for Complex<T>
where
    T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.real.partial_cmp(&other.real) {
            Some(Ordering::Equal) => self.imag.partial_cmp(&other.imag),
            result => result,
        }
    }
}

impl<T> Ord for Complex<T>
where
    T: PartialOrd + Eq
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

///加法+
/// # EXamples
/// ```
/// use algori::structure::Complex;
/// let a = Complex{real: 1, imag: 2};
/// let b = Complex{real: 2 ,imag: 3};
/// let c = a + b;
/// println!("c is {:?}", c);
/// ```
impl<T> Add for Complex<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Complex<T>;

    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}
///减法-
impl<T> Sub for Complex<T>
where
    T: Sub<Output = T> + Copy
{
    type Output = Complex<T>;

    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}
///乘法*
impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + std::ops::Sub<Output = T>
{
    type Output = Complex<T>;

    fn mul(self, other: Complex<T>) -> Complex<T> {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex { real, imag }
    }
}
///除法/
impl<T> Div for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy
{
    type Output = Complex<T>;

    fn div(self, other: Complex<T>) -> Complex<T> {
        let denom = other.real * other.real + other.imag * other.imag;
        let real = (self.real * other.real + self.imag * other.imag) / denom;
        let imag = (self.imag * other.real - self.real * other.imag) / denom;
        Complex { real, imag }
    }
}

impl<T> Complex<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy
{
    ///创建一个复数
    pub fn new(real: T, imag: T) -> Complex<T> {
        Complex { real, imag }
    }

}
