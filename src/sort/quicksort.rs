/// 快速排序
///
/// # Examples
///
/// ```
/// use algori::sort::quicksort;
///
/// let mut a = [7, 3, 5, 1, 9, 65, 65, 4, 6];
/// quicksort(&mut a);
/// assert_eq!(a, [1, 3, 4, 5, 6, 7, 9, 65, 65]);
/// ```
 
pub fn quicksort<T: Ord +Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let pivot = &arr[arr.len() - 1].clone();
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] < *pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}
