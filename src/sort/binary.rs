///二分插入排序
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::binary_sort;
///let c = binary_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
pub fn sort<T>(array: &mut[T]) ->() where
    T: Ord, {
    for i in 1..(*array).len() {
	match crate::search::binary_search(&(*array)[0..i], &array[i]) {
	    Ok(_) => continue,
	    Err(point) => {
		for j in 0..(i - point) {
		    (*array).swap(i -j - 1, i - j);
		}
	    }
	}
    }
    ///当最后一个元素与前面相同时会出现问题,所以执行一次插入排序 损耗为n
    if (*array)[(*array).len() - 1] < (*array)[(*array).len() - 2] {
	let mut i = (*array).len() - 1;
	while i > 0 && (*array)[i - 1] >=(*array)[i] {
	    (*array).swap(i,i - 1);
	    i -= 1;
	}
    }
}
