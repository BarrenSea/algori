///快速排序
pub fn quicksort<T: Ord>(arr: &mut [T]) ->() {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr.len() / 2;
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while arr[left] < arr[pivot] {
            left += 1;
        }
        while arr[right] > arr[pivot] {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if right > 0 {
        quicksort(&mut arr[0..right + 1]);
    }
    if left < arr.len() {
        quicksort(&mut arr[left..]);
    }
}


