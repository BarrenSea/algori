///二分插入排序
pub fn sort<T>(array: &mut[T]) ->() where
    T: Ord, {
    for i in 1..(*array).len() {
	match crate::search::binary_search(&(*array)[0..i], &array[i]) {
	    Ok(_) => continue,
	    Err(point) => {
		for j in 0..(i - point) {
		    (*array).swap(i -j, i - j - 1);
		}
	    }
	}
    }
}
