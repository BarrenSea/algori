/// # Determine a slice if it is ordered
/// *Feature* : Use a Function to check if a slice is sorted or not
///
/// *Return* : if is sorted or not sorted, return true , if the element cant compare, return false
///
/// # Use
/// ```
/// use algori::sorting::{is_sorted,insertion_sort};
/// let mut a = [1,3,2,0,123,1,1,4634,341,2312452,351];
/// assert_eq!(is_sorted(&mut a, |a,b|a <=b),false);
///
/// insertion_sort(&mut a,|a,b| a <=b);
/// assert_eq!(is_sorted(&mut a,|a,b|a<=b),true);
/// ```
pub fn is_sorted<'a, T>(array: &'a [T], compare: fn(&T, &T) -> bool) -> bool
where
    T: PartialOrd + 'a,
{
    if array.len() == 0 {
        return true;
    }
    for i in 0..array.len() - 1 {
        match compare(&array[i], &array[i + 1]) {
            true => {
                continue;
            }
            false => {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod issorted_test {
    use super::is_sorted;
    #[test]
    fn sorted() -> () {
        let a = [1, 2, 3, 4, 5, 6, 111, 321312321, 623123124];
        assert_eq!(is_sorted(&a, |a, b| a <= b), true);
    }
    #[test]
    fn unsorted() -> () {
        let a = [2, 0, 3, 0, 4, 9, 323, 1, 4, 7, 1, 233, 6, 7];
        assert_eq!(is_sorted(&a, |a, b| a <= b), false);
    }
    #[test]
    fn char_compare() -> () {
        let a: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        assert_eq!(is_sorted(&a, |a, b| a <= b), true);
    }
    #[test]
    fn reverse() -> () {
        let a = [7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(is_sorted(&a, |a, b| a >= b), true);
    }
}
