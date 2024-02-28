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
fn max<T: PartialOrd>(arr: &mut [T], i: usize) {
    let mut n: usize = i ;
    //循还下滤
    loop {
        let mut largest:usize = n;
	//左子结点
        let left:usize = 2 * n + 1;
	//右子结点
        let right:usize = 2 * n + 2;
	//左子结点大于目前结点则交换下标
        if left < arr.len() && arr[left] > arr[largest] {
            largest = left;
        }
	//右子结点大于目前结点则交换下标
        if right < arr.len() && arr[right] > arr[largest] {
            largest = right;
        }
	//到这里则对一个结点及其子结点的下标排好了序
	//若下标发生了变换 则交换父结点与最大的子结点
        if largest != n {
            arr.swap(n, largest);
            n = largest;
        } else {
	    //下滤完成
            break;
        }
    }
}

//建堆
pub fn build_max<T: PartialOrd>(arr: &mut [T]) {

    let n = arr.len();
    // n/2为拥有子结点的下标最大的
    for i in (0..n / 2).rev() {
        max(arr, i);
    }
}
///大根堆排序
///
///获取一个可变引用并排序
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    build_max(arr);
    for i in (0..n).rev() {
        arr.swap(0, i);
        max(&mut arr[..i], 0);
    }
}
