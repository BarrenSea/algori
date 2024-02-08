///同时找min和max
///只需要3(n/2)的代价
///返回包含(最小值下标,最大值下标)的元组
///# Examples
///```
///let mut  a  = [7,3,5,1,9,65,4,5];
///use algori::search::min_and_max;
///let c = min_and_max(&a);
///assert_eq!(c,Some((3,5)));
///```
pub fn find_min_max<T: PartialOrd + Copy>(arr: &[T]) -> Option<(usize, usize)> {
    if arr.is_empty() {
        return None;
    }

    let mut min = arr[0];
    let mut max = arr[0];
    let mut min_index = 0;
    let mut max_index = 0;
    let mut i = 1;
    if arr.len() % 2 == 0 {
        if arr[0] > arr[1] {
            max = arr[0];
            min = arr[1];
	    min_index = 1;
	    max_index = 0;
        } else {
            max = arr[1];
            min = arr[0];
	    min_index = 0;
	    max_index = 1;
        }
        i = 2;
    }

    while i < arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            if arr[i] > max {
                max = arr[i];
		max_index = i;
            }
            if arr[i + 1] < min {
                min = arr[i + 1];
		min_index = i + 1;
            }
        } else {
            if arr[i + 1] > max {
                max = arr[i + 1];
		max_index = i + 1;
            }
            if arr[i] < min {
                min = arr[i];
		min_index = i;
            }
        }
        i += 2;
    }

    Some((min_index,max_index))
}

