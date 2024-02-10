///欧几里德最大公约数
///# Example
///```
///use algori::math::gcd;
///let b = gcd(18,9);
///assert_eq!(b,9);
///```
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Rem<Output = T> + std::ops::Div<Output = T> + std::cmp::PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8>,
{
    let zero = T::from(0u8);
    let mut a = a;
    let mut b = b;
    while b != zero {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

