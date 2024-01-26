///线性查找数组中的最小值
pub fn min<T: Ord>(array: &[T]) ->usize {
    let mut min: &T = &array[0];
    let mut min_index: usize = 0;
    for i in 1..(*array).len() {
	if (*array)[i] < *min {
	    min = &array[i];
	    min_index = i;
	}
    }
    min_index
}
