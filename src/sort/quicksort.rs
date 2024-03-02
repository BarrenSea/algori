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
pub fn quicksort<T: PartialOrd>(array: &mut [T]) -> () {
    let len = array.len();
    if len > 1 {
	let pivot = partition(array);
	quicksort(&mut array[0..pivot]);
	quicksort(&mut array[pivot + 1 ..len]);
    }
}

fn partition<T: PartialOrd>(array: &mut[T]) -> usize {
    let mut less_index = 0;

    let pivot_index = array.len() - 1;
    for j in 0..=array.len() -2 {
	if array[j] <= array[pivot_index] {
	    array.swap(less_index,j);
	    less_index += 1;
	}
    }

    array.swap(less_index,array.len() - 1);
    less_index
}
