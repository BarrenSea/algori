use super::NumOps;
use core::ops::{Add, Div, Mul, Sub};

/// impl<'a,'b,T: Clone + NumOps> Method<&'a Complex<T>> for &'b Complex<T>
macro_rules! impl_ref_ref {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + NumOps> $imp<&'a Complex<T>> for &'b Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &'a Complex<T>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

/// impl<'a,T: Clone + NumOps>  Complex<T> for &'a Complex<T>
macro_rules! impl_val_ref {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + NumOps> $imp<&'a Complex<T>> for Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &'a Complex<T>) -> Self::Output {
                self.$method(other.clone())
            }
        }
    };
}

/// impl<'a,T: Clone + NumOps> &'a Complex<T> for  Complex<T>
macro_rules! impl_ref_val {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + NumOps> $imp<Complex<T>> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: Complex<T>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! impl_all {
    (impl $imp:ident ,$method: ident) => {
        impl_ref_val!(impl $imp, $method);
        impl_ref_ref!(impl $imp, $method);
        impl_val_ref!(impl $imp, $method);
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
/// Complex
pub struct Complex<T> {
    /// Real
    pub re: T,
    /// Imaginary
    pub im: T,
}

impl<T> Complex<T> {
    #[inline]
    /// Create a new `Complex`
    pub fn new(re: T, im: T) -> Self {
        return Complex { re, im };
    }
}

/// (a+bi) + (c+di) = (a+c)+(b+d)i
impl<T: NumOps> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    #[inline]
    fn add(self, rhs: Complex<T>) -> Self::Output {
        return Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        };
    }
}

impl_all!(impl Add, add);

/// (a + bi) - (c + di) = (a - c ) + (b - d)i
impl<T: NumOps> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    #[inline]
    fn sub(self, rhs: Complex<T>) -> Self::Output {
        return Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        };
    }
}

impl_all!(impl Sub, sub);

/// (a+bi) * (c+di) = (a*c - b*d)+(a*d + b*c)i
impl<T: NumOps + Clone> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    #[inline]
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        return Complex {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + self.im * rhs.re,
        };
    }
}

impl_all!(impl Mul, mul);

/// a+bi / c+di = (a * c + b * d)/ (c*c + d*d) + ((b*c - a*d)/(c*c + d*d))i
impl<T: NumOps + Clone> Div<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    #[inline]
    fn div(self, rhs: Complex<T>) -> Self::Output {
        let tmp = rhs.re.clone() * rhs.re.clone() + rhs.im.clone() * rhs.im.clone();
        let re =
            (self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone()) / tmp.clone();
        let im = (self.im.clone() * rhs.re.clone() - self.re.clone() * rhs.im.clone()) / tmp;
        return Complex { re, im };
    }
}

impl_all!(impl Div, div);

/// a+bi -> (a,b)
impl<T> core::fmt::Display for Complex<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(f, "({},{})", self.re, self.im);
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        return Self {
            re: value.0,
            im: value.1,
        };
    }
}

#[cfg(test)]
mod max_substring {
    use std::hash::Hasher;

    use super::*;
    #[test]
    fn create_new_complex() {
        let a = Complex::new(1, 1);
        assert_eq!(Complex { re: 1, im: 1 }, a);
    }
    #[test]
    fn add() {
        let a: Complex<i32> = (20, 30).into();
        let b: Complex<i32> = (992, 8770).into();
        assert_eq!(Complex { re: 1012, im: 8800 }, a + b);
    }
    #[test]
    fn mul() {
        let a: Complex<i32> = (20, 30).into();
        let b: Complex<i32> = (992, 8770).into();
        assert_eq!(
            Complex {
                re: -243260,
                im: 205160
            },
            a * b
        );
    }
    #[test]
    fn div() {
        let a: Complex<i32> = (20, 30).into();
        let b: Complex<i32> = (992, 8770).into();
        assert_eq!(Complex { re: 217, im: 112 }, b / a);
    }
    #[test]
    fn sub() {
        let a: Complex<i32> = (20, 30).into();
        let b: Complex<i32> = (992, 8770).into();
        assert_eq!(Complex { re: 972, im: 8740 }, b - a);
    }
    #[test]
    fn hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let a: Complex<i32> = (20, 30).into();
        let b: Complex<i32> = (20, 40).into();
        let c: Complex<i32> = (20, 30).into();
        assert_eq!(calculate_hash(&a), calculate_hash(&c));
        assert_eq!(calculate_hash(&a) == calculate_hash(&b), false);
    }
}
