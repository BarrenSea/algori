///离散傅立叶变换
#[allow(non_snake_case)]
mod DFT;

///欧几里德最大公约数
mod gcd;
pub use self::DFT::dft as dft;
pub use self::gcd::gcd as gcd;
