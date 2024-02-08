///二分查找
///O(log_{2}n)
///找到时返回下标,未找到时返回插入点
///# Examples
///```
///use algori::sort::pdqsort;
///let mut  a  = [7,3,5,1,9,65,4,5];
///pdqsort(&mut a); ///二分查找需要有序队列
///use algori::search::binary_search;
///let c = binary_search(&a,&7);
///assert_eq!(c,Ok(5));
///```
pub fn search<T>(array: &[T], key: &T) -> Result<usize,usize> where
    T: Ord,
{

    let mut left = 0;
    let mut right = array.len();
    while left < right {
	let mid = (left + right) / 2;
	if array[mid] < *key {
	    left =  mid + 1;
	} else if array[mid] > *key {
	    right = mid;
	} else {
	    return Result::Ok(mid);
	}
    }
    Result::Err(left)
}
