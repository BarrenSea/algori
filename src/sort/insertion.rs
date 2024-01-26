///插入排序
///Θ(n^2)

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
