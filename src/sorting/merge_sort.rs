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
/// # Example
/// ```
/// use algori::sorting::{merge_sort,is_sorted};
/// let mut a = [2,3,1,34,15,8,0,7,4,3,21,4,6,7,4,2341,321,41231,312,62];
/// merge_sort(&mut a ,|a,b| a<=b);
/// assert_eq!(is_sorted(&mut a ,|a,b| a<=b),true);
/// ```
#[cfg(not(feature = "no_std"))]
pub fn merge_sort<T: Clone>(array: &mut [T], comparator: fn(&T, &T) -> bool) {
    if array.len() > 1 {
        if array.len() > 1 {
            let mid = array.len() / 2;
            merge_sort(&mut array[..mid], comparator);
            merge_sort(&mut array[mid..], comparator);
            merge(array, mid, comparator);
        }
    }
}
#[cfg(not(feature = "no_std"))]
#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;
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
