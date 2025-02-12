mod complex;
mod dft;
mod pi;
pub use complex::*;
use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::ops::{Add, Div, Mul, Rem, Sub};
#[cfg(not(feature = "no_std"))]
pub use dft::*;
pub use pi::*;
pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}
pub trait NumCmp<Rhs = Self>: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}
impl<T> NumCmp for T where T: PartialEq + PartialOrd + Eq + Ord {}

/// is_even
/// 判断数字是否为偶数
/// # Example
/// ```
/// use algori::math::is_even;
/// assert_eq!(is_even(12),true);
/// assert_eq!(is_even(99),false);
/// ```
pub fn is_even<T>(number: T) -> bool
where
    T: NumOps + NumCmp + From<u8>,
{
    number % T::from(2) == T::from(0)
}

/// is_odd
/// 判断数字是否为奇数
/// # Example
/// ```
/// use algori::math::is_odd;
/// assert_eq!(is_odd(12),false);
/// assert_eq!(is_odd(99),true);
/// ```
pub fn is_odd<T>(number: T) -> bool
where
    T: NumOps + NumCmp + From<u8>,
{
    !is_even(number)
}

#[cfg(test)]
mod test {
    use crate::math::*;
    #[test]
    fn even() {
        assert_eq!(is_even(12), true);
        assert_eq!(is_even(11), false);
    }
    #[test]
    fn odd() {
        assert_eq!(is_odd(12), false);
        assert_eq!(is_odd(11), true);
    }
}
