///下滤
///复杂度为O(logn)
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::heap_max_sort;
///let c = heap_max_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
fn max<T: Ord>(arr: &mut [T], n: usize, mut i: usize) {
    //循还下滤
    loop {
	//当前结点下标
        let mut largest:usize = i;
	//左子结点
        let left:usize = 2 * i + 1;
	//右子结点
        let right:usize = 2 * i + 2;
	//左子结点大于目前结点则交换下标
        if left < n && arr[left] > arr[largest] {
            largest = left;
        }
	//右子结点大于目前结点则交换下标
        if right < n && arr[right] > arr[largest] {
            largest = right;
        }
	//到这里则对一个结点及其子结点的下标排好了序
	//若下标发生了变换 则交换父结点与最大的子结点
        if largest != i {
            arr.swap(i, largest);
            i = largest;
        } else {
	    //下滤完成
            break;
        }
    }
}

//建堆
fn build_max<T: Ord>(arr: &mut [T]) {

    let n = arr.len();
    // n/2为拥有子结点的下标最大的
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


