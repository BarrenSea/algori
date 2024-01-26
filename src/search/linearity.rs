///线性查找
///当找到时返回下标,否则Option::None
pub fn search<T: std::cmp::PartialEq, E>(a: &[T], v: T) -> Option<usize> {
    let len = (*a).len();
    let mut i =  0;
    while i < len {
        if (*a)[i] != v {
            i = i + 1;
        } else {return Option::Some(i)};
    }
    Option::None
}


