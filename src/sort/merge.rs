fn merge<T>(left: &[T], right: &[T]) -> Vec<T> where
    T: std::cmp::PartialOrd + Copy,
{

    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    let left_len: usize = left.len();
    let right_len: usize = right.len();
    while left_index < left_len && right_index < right_len {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    while left_index < left_len {
        result.push(left[left_index]);
        left_index += 1;
    }

    while right_index < right_len {
        result.push(right[right_index]);
        right_index += 1;
    }

    result
}

///分治排序
///Θ(nlog_{2}n)
///# Examples
///```
///
///let mut  a  = vec!(7,3,5,1,9,65,65,4,6,6);
///
///use algori::sort::merge_sort;
///let c = merge_sort(&mut a);
///assert_eq!(c,[1,3,4,5,6,6,7,9,65,65]);
///```
pub fn sort<T>(arr: &mut Vec<T>) -> Vec<T> where
    T: std::cmp::PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return arr.clone();
    }

    let mid = arr.len() / 2;
    let left = sort(&mut arr[0..mid].to_vec());
    let right = sort(&mut arr[mid..].to_vec());
    merge(&left, &right)
}

