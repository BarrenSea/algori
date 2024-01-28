///同时找min和max
///只需要3(n/2)的代价
pub fn find_min_max<T: PartialOrd + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.is_empty() {
        return None;
    }

    let mut min = arr[0];
    let mut max = arr[0];

    let mut i = 1;
    if arr.len() % 2 == 0 {
        if arr[0] > arr[1] {
            max = arr[0];
            min = arr[1];
        } else {
            max = arr[1];
            min = arr[0];
        }
        i = 2;
    }

    while i < arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            if arr[i] > max {
                max = arr[i];
            }
            if arr[i + 1] < min {
                min = arr[i + 1];
            }
        } else {
            if arr[i + 1] > max {
                max = arr[i + 1];
            }
            if arr[i] < min {
                min = arr[i];
            }
        }
        i += 2;
    }

    Some((min, max))
}

