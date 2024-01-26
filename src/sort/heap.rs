///下滤
///复杂度为O(logn)
fn max<T: Ord>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != i {
            arr.swap(i, largest);
            i = largest;
        } else {
            break;
        }
    }
}

fn build_max<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        max(arr, n, i);
    }
}
///大根堆排序
///
///获取一个可变引用并排序
pub fn max_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    build_max(arr);

    for i in (0..n).rev() {
        arr.swap(0, i);
        max(arr, i, 0);
    }
}


/// 上滤
/// 复杂度为O(logn)
fn min<T: Ord>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut smallest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] < arr[smallest] {
            smallest = left;
        }

        if right < n && arr[right] < arr[smallest] {
            smallest = right;
        }

        if smallest != i {
            arr.swap(i, smallest);
            i = smallest;
        } else {
            break;
        }
    }
}

fn build_min<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        min(arr, n, i);
    }
}

/// 小根堆排序
///
/// 获取一个可变引用并排序
pub fn min_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    build_min(arr);

    for i in (0..n).rev() {
        arr.swap(0, i);
        min(arr, i, 0);
    }
}


