///选择排序
///O(n^2)
/// # Examples
///
/// ```
/// use algori::sort::selection_sort;
///
/// let mut a = [7, 3, 5, 1, 9, 65, 65, 4, 6];
/// selection_sort(&mut a);
/// assert_eq!(a, [1, 3, 4, 5, 6, 7, 9, 65, 65]);
/// ```
pub fn sort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let len = (*a).len();
    for i in 0..=len {
        for t in i+1..len {
            if (*a)[t] <= a[i] {
                a.swap(t,i);
            }
        }
    }
}




