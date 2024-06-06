/// # Insertion_Sort
pub fn insertion_sort<T>(array: &mut [T]) -> ()
where
    T: PartialOrd,
{
    for point in 1..array.len() {
        let mut current_point: usize = point;
        while current_point > 0 && array[current_point] < array[current_point - 1] {
            array.swap(current_point, current_point - 1);
            current_point -= 1;
        }
    }
}

/// # Reverse Insertion_Sort
pub fn reverse_insertion_sort<T>(array: &mut [T]) -> ()
where
    T: PartialOrd,
{
    for point in 1..array.len() {
        let mut current_point: usize = point;
        while current_point > 0 && array[current_point] > array[current_point - 1] {
            array.swap(current_point, current_point - 1);
            current_point -= 1;
        }
    }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::super::is_sorted;
    use super::{insertion_sort, reverse_insertion_sort};
    #[test]
    fn empty() -> () {
        let mut a: [i32; 0] = [];
        insertion_sort(&mut a);

        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }

    #[test]
    fn one_element() -> () {
        let mut a: [i32; 1] = [1];
        insertion_sort(&mut a);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn positive() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        insertion_sort(&mut a);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn reverse() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        reverse_insertion_sort(&mut a);
        assert_eq!(is_sorted(&mut a, |a, b| a >= b), true);
    }
}
