
///泡泡排序
///O(n^2)
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::bubble_sort;
///let c = bubble_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
pub fn sort<T: std::cmp::PartialOrd>(a: &mut[T]){
    for i in 0..(*a).len(){
	for j in 0..(*a).len() -i -1 {
	    if (*a)[j] > (*a)[j + 1] {
		(*a).swap(j, j + 1);
	    }
	}
    }

}

