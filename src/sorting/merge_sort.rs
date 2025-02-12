#[cfg(feature = "no_std")]
use crate::utils::rotate;

#[cfg(feature = "no_std")]
fn merge<T>(array: &mut [T], mid: usize, comparator: fn(&T, &T) -> bool) {
    let len = array.len();
    let mut left = 0;
    let mut right = mid;

    while left < right && right < len {
        if comparator(&array[left], &array[right]) {
            left += 1;
        }
        // 此时comparator(array[left],array[right]) = false
        else {
            let tmp = right;
            while right < len && comparator(&array[left], &array[right]) == false {
                right += 1;
            }

            // 此时comparator(array[left],array[right]) = true
            // 在right到tmp之间的元素是需要与i到tmp旋转的
            let rotate_mid = tmp - left;
            let slice = &mut array[left..right];
            rotate(slice, rotate_mid).unwrap();
            // 更新左指针位置
            left += right - tmp;
        }
    }
}

#[cfg(not(feature = "no_std"))]
fn merge<T: Clone>(array: &mut [T], mid: usize, comparator: fn(&T, &T) -> bool) {
    let left = array[..mid].to_vec();
    let right = array[mid..].to_vec();
    let mut left_index = 0;
    let mut right_index = 0;
    let mut index = 0;

    while left_index < left.len() && right_index < right.len() {
        if comparator(&left[left_index], &right[right_index]) {
            array[index] = left[left_index].clone();
            left_index += 1;
        } else {
            array[index] = right[right_index].clone();
            right_index += 1;
        }
        index += 1;
    }

    while left_index < left.len() {
        array[index] = left[left_index].clone();
        left_index += 1;
        index += 1;
    }

    while right_index < right.len() {
        array[index] = right[right_index].clone();
        right_index += 1;
        index += 1;
    }
}

/// Merge Sort
/// - no_std启用时: 采用旋转元素与自底向上的归并排序
/// - std启用时: 原始的归并排序
/// # Example
/// ```
/// use algori::sorting::{merge_sort,is_sorted};
/// let mut a = [2,3,1,34,15,8,0,7,4,3,21,4,6,7,4,2341,321,41231,312,62];
/// merge_sort(&mut a ,|a,b| a<=b);
/// assert_eq!([0, 1, 2, 3, 3, 4, 4, 4, 6, 7, 7, 8, 15, 21, 34, 62, 312, 321, 2341, 41231],a)
/// ```
#[cfg(feature = "no_std")]
pub fn merge_sort<T>(array: &mut [T], comparator: fn(&T, &T) -> bool) {
    let n = array.len();
    let mut size = 1;

    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = left + size;
            let right = core::cmp::min(left + 2 * size, n);
            if mid < right {
                merge(&mut array[left..right], mid - left, comparator);
            }
        }
        size *= 2;
    }
}

#[cfg(not(feature = "no_std"))]
pub fn merge_sort<T: Clone>(array: &mut [T], comparator: fn(&T, &T) -> bool) {
    let n = array.len();
    let mut size = 1;

    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = left + size;
            let right = core::cmp::min(left + 2 * size, n);
            if mid < right {
                merge(&mut array[left..right], mid - left, comparator);
            }
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn test_merge() {
        let mut a = [0, 10, 20, 30, 40, 11, 12, 21, 22, 33, 34, 35, 39, 41];
        merge(&mut a, 5, |a, b| a <= b);
        assert_eq!(a, [0, 10, 11, 12, 20, 21, 22, 30, 33, 34, 35, 39, 40, 41]);
    }
    #[test]
    fn test_merge_sort() {
        let mut a = [
            2, 3, 1, 34, 15, 8, 0, 7, 4, 3, 21, 4, 6, 7, 4, 2341, 321, 41231, 312, 62,
        ];
        merge_sort(&mut a, |a, b| a <= b);
        assert!(is_sorted(&a, |a, b| a <= b));
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut a: [i32; 0] = [];
        merge_sort(&mut a, |a, b| a <= b);
        assert!(is_sorted(&a, |a, b| a <= b));
    }

    #[test]
    fn test_merge_sort_single_element() {
        let mut a = [1];
        merge_sort(&mut a, |a, b| a <= b);
        assert!(is_sorted(&a, |a, b| a <= b));
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let mut a = [1, 2, 3, 4, 5];
        merge_sort(&mut a, |a, b| a <= b);
        assert!(is_sorted(&a, |a, b| a <= b));
    }

    #[test]
    fn test_merge_sort_reverse_sorted() {
        let mut a = [5, 4, 3, 2, 1];
        merge_sort(&mut a, |a, b| a <= b);
        assert!(is_sorted(&a, |a, b| a <= b));
    }
}
