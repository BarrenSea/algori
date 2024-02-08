///线性查找
///当找到时返回下标,否则Option::None
///# Examples
///```
///let mut  a  = [7,3,5,1,9,65,4,5];
///use algori::search::linearity_search;
///let c = linearity_search::<i32>(&a,&65); 
///assert_eq!(c,Some(5));
///```
pub fn search<T: std::cmp::PartialEq>(a: &[T], v: &T) -> Option<usize> {
    let len = (*a).len();
    let mut i =  0;
    while i < len {
        if (*a)[i] != *v {
            i = i + 1;
        } else {return Option::Some(i)};
    }
    Option::None
}


