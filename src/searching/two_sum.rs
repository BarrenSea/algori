use core::hash::Hash;
use std::collections::HashMap;
/// 两数之和
/// 在给定序列中寻找和为目标值的两个数的下标
/// ```
/// use algori::searching::two_sum;
/// let a = [2,7,11,15];
/// assert_eq!(two_sum(&a,&9),Ok((0,1)));
/// ```
pub fn two_sum<T: Hash + Eq + Clone + std::ops::Sub<Output = T>>(
    array: &[T],
    target: &T,
) -> Result<(usize, usize), ()> {
    let mut table: HashMap<&T, usize> = HashMap::new();
    for i in 0..array.len() {
        let sub = target.clone() - array[i].clone();
        match table.get(&sub) {
            Some(index) => {
                return Ok((*index, i));
            }
            None => {
                table.insert(&array[i], i);
            }
        }
    }
    return Err(());
}
