///二分查找
///O(log_{2}n)
///找到时返回下标,未找到时返回插入点
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
