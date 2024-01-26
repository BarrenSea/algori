

///泡泡排序
///O(n^2)
pub fn sort<T: std::cmp::PartialOrd>(a: &mut[T]){
    for i in 0..(*a).len(){
	for j in 0..(*a).len() -i -1 {
	    if (*a)[j] > (*a)[j + 1] {
		(*a).swap(j, j + 1);
	    }
	}
    }

}

