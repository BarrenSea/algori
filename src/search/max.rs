///线性查找数组中的最大值
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,4,5];
///
///use algori::search::max_search;
///let c = max_search(&a);
///assert_eq!(c,5);
///```
pub fn max<T: Ord>(array: &[T]) ->usize {
    let mut max: &T = &array[0];
    let mut max_index: usize = 0;
    for i in 1..(*array).len() {
	if (*array)[i] > *max {
	    max = &array[i];
	    max_index = i;
	}
    }
    max_index
}
