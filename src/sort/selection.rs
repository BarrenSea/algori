///选择排序
///O(n^2)
pub fn sort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let len = (*a).len();
    for i in 0..=len {
        for t in i+1..len {
            if (*a)[t] <= a[i] {
                a.swap(t,i);
            }
        }
    }
}




