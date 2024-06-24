/// # Binary_Search
/// binary_search function receives a `positive`(PartialOrd) array and an element
/// - If this array contains this element, return `Ok(the index of the element)`
/// - If this array not contains this element, return `Err(the index of the insertion point)`
/// - `Insert point` is the index after the largest element smaller than the given element
/// # Example
/// ```
/// use algori::searching::binary_search;
/// let a = [0,1,3,4,9,20];
/// assert_eq!(Err(5),binary_search(&a,&10));
/// ```
pub fn binary_search<T: PartialOrd>(array: &[T], element: &T) -> Result<usize, usize> {
    let mut left_idx: usize = 0;
    let mut right_idx: usize = array.len();
    let mut mid_idx: usize = 0;
    while left_idx < right_idx {
        mid_idx = (left_idx + right_idx) / 2;
        if &array[mid_idx] < element {
            left_idx = mid_idx + 1;
        } else if &array[mid_idx] > element {
            right_idx = mid_idx;
        } else {
            return Ok(mid_idx);
        }
    }
    return Err(left_idx);
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (arr, element, expected) = $test_case;
                    assert_eq!(binary_search(arr, element), expected);
                }
            )*
        };
    }
    test_cases! {
    empty: (&[],&0,Err(0)),
    one_element: (&[1],&1,Ok(0)),
    one_element_not_found: (&[1],&0,Err(0)),
    string_one: (&["a","b"],&"a",Ok(0)),
    string: (&["a","b","c","e","g"],&"c",Ok(2)),
    string_now_found: (&["a","b","c","e","g"],&"d",Err(3)),
    }
}
