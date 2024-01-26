///线性查找数组中的最大值
pub fn max<T: Ord>(array: &[T]) ->usize {
    let mut max: &T = &array[0];
    let mut max_index: usize = 0;
    for i in 1..(*array).len() {
	if (*array)[i] > *max {
	    max = &array[i];
	    max_index = i;
	}
    }
    (max_index)
}
