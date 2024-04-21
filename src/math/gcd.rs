///欧几里德最大公约数
///# Example
///```
///use algori::math::gcd;
///let b = gcd(18,9).await;
///assert_eq!(b,9);
///```
pub async fn gcd<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Copy + std::convert::From<u8>,
{
    let mut x = a;
    let mut y = b;
    while y != T::from(0) {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x
}
