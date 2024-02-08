
fn counting_sort(arr: &mut [usize], exp: usize) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        count[(num / exp) % 10] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    let mut i = n as i32 - 1;
    while i >= 0 {
        let num = arr[i as usize];
        output[count[(num / exp) % 10] - 1] = num;
        count[(num / exp) % 10] -= 1;
        i -= 1;
    }

    for i in 0..n {
        arr[i] = output[i];
    }
}

///基数排序
///Θ(d(n+k))
/// # Examples
///
/// ```
/// use algori::sort::radix_sort;
///
/// let mut a = [7, 3, 5, 1, 9, 65, 65, 4, 6];
/// radix_sort(&mut a);
/// assert_eq!(a, [1, 3, 4, 5, 6, 7, 9, 65, 65]);
/// ```
pub fn radix_sort(arr: &mut [usize]) {
    let max = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1;
    while max / exp > 0 {
        counting_sort(arr, exp);
        exp *= 10;
    }
}

