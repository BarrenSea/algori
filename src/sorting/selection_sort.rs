/// # Selection Sort
/// # Example
/// ```
/// use algori::sorting::{selection_sort,is_sorted};
/// let mut a = [2,3,1,34,15,8,0,7,4,3,21,4,6,7,4,2341,321,41231,312,62];
/// selection_sort(&mut a,|a,b| a <= b);
/// is_sorted(&mut a,|a,b| a<=b);
/// ```
pub fn selection_sort<T>(array: &mut [T],comparator: fn(&T,&T) -> bool) ->()
where T: PartialOrd
{
    for point in 0..array.len() {
	let mut better:usize = point;
	for index in point..array.len() {
	    if comparator(&array[index],&array[better]) {
		better = index;
	    }
	}
	array.swap(better,point);
    }
}


#[cfg(test)]
mod selection_sort_tests {
    use super::super::is_sorted;
    use super::selection_sort;
    #[test]
    fn empty() -> () {
        let mut a: [i32; 0] = [];
        selection_sort(&mut a,|a,b| a<=b);

        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }

    #[test]
    fn one_element() -> () {
        let mut a: [i32; 1] = [1];
        selection_sort(&mut a,|a,b| a<=b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
    #[test]
    fn positive() -> () {
        let mut a = [1, 123, 123, 12, 4234, 42, 1123, 123, 15112, 312];
        selection_sort(&mut a,|a,b| a<=b);
        assert_eq!(is_sorted(&mut a, |a, b| a <= b), true);
    }
}
