///插入排序
///Θ(n^2)
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::insertion_sort;
///let c = insertion_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
pub fn sort<T: std::cmp::PartialOrd>(arr: &mut[T]) {
    
    let len = (*arr).len();
    for index in 1..len {
        let mut i = index ;
        while i > 0 && arr[i - 1] >= arr[i] {   
            (*arr).swap(i,i - 1);
            i -= 1;
        }
    }
}
