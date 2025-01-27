/// Insertion Sort
/// # Example
/// ```
/// use algori::sorting::{insertion_sort,is_sorted};
/// let mut a = [2,3,1,34,15,8,0,7,4,3,21,4,6,7,4,2341,321,41231,312,62];
/// insertion_sort(&mut a,|a,b| a <= b);
/// assert_eq!(is_sorted(&mut a,|a,b| a<=b),true);
/// ```
pub fn insertion_sort<T>(array: &mut [T], comparator: fn(&T, &T) -> bool) -> () {
    for point in 1..array.len() {
        let mut current_point: usize = point;
        while current_point > 0 && comparator(&array[current_point], &array[current_point - 1]) {
            array.swap(current_point, current_point - 1);
            current_point -= 1;
        }
    }
}

use crate::searching::binary_search;
/// Binary Sort
/// # Example
/// ```
/// use algori::sorting::{binary_sort,is_sorted};
/// let mut a = [2,3,1,34,15,8,0,7,4,3,21,4,6,7,4,2341,321,41231,312,62];
/// binary_sort(&mut a,|a,b| a<=b);
/// assert_eq!(is_sorted(&mut a,|a,b| a<=b),true);
/// ```
pub fn binary_sort<T>(array: &mut [T], comparator: fn(&T, &T) -> bool) -> () {
    for point in 1..array.len() {
        let mut current_point: usize = point;
        match binary_search(&array[0..point], &array[point], comparator) {
            Ok(index) => {
                while current_point > index {
                    array.swap(current_point, current_point - 1);
                    current_point -= 1;
                }
            }
            Err(index) => {
                while current_point > index {
                    array.swap(current_point, current_point - 1);
                    current_point -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::super::is_sorted;
    use super::insertion_sort;
    #[test]
    fn empty() -> () {
        let mut a: [i32; 0] = [];
        insertion_sort(&mut a, |a, b| a <= b);

        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }

    #[test]
    fn one_element() -> () {
        let mut a: [i32; 1] = [1];
        insertion_sort(&mut a, |a, b| a <= b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn positive() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        insertion_sort(&mut a, |a, b| a <= b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn reverse() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        insertion_sort(&mut a, |a, b| a >= b);
        assert_eq!(is_sorted(&mut a, |a, b| a >= b), true);
    }
}

#[cfg(test)]
mod binary_sort_tests {
    use crate::sorting::binary_sort;
    use crate::sorting::is_sorted;
    #[test]
    fn empty() -> () {
        let mut a: [i32; 0] = [];
        binary_sort(&mut a, |a, b| a <= b);

        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }

    #[test]
    fn one_element() -> () {
        let mut a: [i32; 1] = [1];
        binary_sort(&mut a, |a, b| a <= b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn positive() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        binary_sort(&mut a, |a, b| a <= b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
}
