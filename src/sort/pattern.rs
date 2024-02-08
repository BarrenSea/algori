

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
///pdqsort
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::pdqsort;
///let c = pdqsort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
pub fn pattern_defeating_quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 16 {
        crate::sort::insertion_sort(arr);
    } else {
        let pivot_index = partition(arr);
        pattern_defeating_quicksort(&mut arr[0..pivot_index]);
        pattern_defeating_quicksort(&mut arr[pivot_index + 1..]);
    }
}

