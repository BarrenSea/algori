mod complex;
mod dft;
pub use complex::*;
pub use dft::*;

use core::ops::{Add, Div, Mul, Rem, Sub};
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
