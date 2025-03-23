//#[cfg(feature = "random")]
//use rand::{self, Rng};

/// test the time of function
/// # Examples
/// ```
/// use algori::test_time;
/// use algori::sorting::insertion_sort;
/// let mut a = [1,4,6,7,2,2,1,4,65,6];
/// test_time!("Insertion Sort",3,insertion_sort(&mut a,|a,b|a<=b));
/// ```
#[macro_export]
macro_rules! test_time {
    ($title:literal, $n:expr, $func:expr) => {
        let mut total_duration = std::time::Duration::new(0, 0);

        for i in 0..$n {
            let now = std::time::Instant::now();
            $func;
            let elapsed = now.elapsed();
            total_duration += elapsed;

            // 格式化纳秒部分为 000_000_000 的形式
            let nanos = elapsed.as_nanos();
            let formatted_nanos = format!("{:09}", nanos)
                .chars()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("_");

            println!(
                "Job:\t{}\tIteration:\t{}\tUsing\t{}\tseconds\t{}\tnanos",
                $title,
                i + 1,
                elapsed.as_secs(),
                formatted_nanos
            );
        }

        // 计算平均耗时
        let avg_secs = total_duration.as_secs() / $n;
        let avg_nanos = (total_duration.as_nanos() / ($n as u128));

        // 格式化平均纳秒部分为 000_000_000 的形式
        let formatted_avg_nanos = format!("{:09}", avg_nanos)
            .chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("_");

        println!(
            "Job:\t{}\tIterations:\t{}\tAverage Time:\t{}\tseconds\t{}\tnanos",
            $title, $n, avg_secs, formatted_avg_nanos
        );
    };
}

fn reverse<T>(array: &mut [T], mut start: usize, mut end: usize) {
    while start < end {
        array.swap(start, end);
        start += 1;
        end -= 1;
    }
}
/// 手摇旋转
/// 旋转以 `mid` 为分界的切片
/// - 若mid超出边界或者位于边界 那么返回Err(())
/// - 若成功返回Ok(())
/// # Example
/// ```
/// use algori::utils::rotate;
/// let mut a = ['a','b','c','d','e','f'];
/// rotate(&mut a, 2);
/// assert_eq!(a,['c','d','e','f','a','b']);
/// ```
pub fn rotate<T>(array: &mut [T], mid: usize) -> Result<(), ()> {
    let len = array.len();
    // 越界Err
    if mid >= len {
        return Err(());
    }
    if mid == 0 {
        return Ok(());
    }
    reverse(array, 0, mid - 1);
    reverse(array, mid, len - 1);
    reverse(array, 0, len - 1);
    return Ok(());
}

#[cfg(test)]
mod rotate_test {
    use crate::utils::rotate;
    // 测试奇数的数组
    #[test]
    fn odd() {
        let mut a = ['a', 'b', 'c', 'd', 'e'];
        rotate(&mut a, 2).unwrap();
        assert_eq!(a, ['c', 'd', 'e', 'a', 'b']);
    }
    // 偶数的数组
    #[test]
    fn even() {
        let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
        rotate(&mut a, 2).unwrap();
        assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b']);
    }
    // 下标越界
    #[test]
    fn over_index() {
        let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
        let result = rotate(&mut a, 6);
        assert_eq!(a, ['a', 'b', 'c', 'd', 'e', 'f']);
        assert_eq!(result, Err(()));
    }
    // 相等
    #[test]
    fn len() {
        let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
        let result = rotate(&mut a, 5);
        assert_eq!(a, ['f', 'a', 'b', 'c', 'd', 'e']);
        assert_eq!(result, Ok(()));
    }
    // 开始
    #[test]
    fn start() {
        let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
        let result = rotate(&mut a, 0);
        assert_eq!(a, ['a', 'b', 'c', 'd', 'e', 'f']);
        assert_eq!(result, Ok(()));
    }
    #[test]
    fn rotate_test1() {
        let mut a = [20, 30, 40, 50, 15];
        rotate(&mut a, 4).unwrap();
        assert_eq!([15, 20, 30, 40, 50], a);
    }
}

/// 寻找最大元素
/// - 返回最大元素的下标
/// # Example
/// ```
/// use algori::utils::max;
/// let a = [0,20,9,2,3,4,1,9,5,3,2];
/// assert_eq!(max(&a),1);
/// ```
pub fn max<T>(array: &[T]) -> usize
where
    T: core::cmp::Ord,
{
    let mut max_index = 0;
    for index in 0..array.len() {
        if array[index] > array[max_index] {
            max_index = index;
        }
    }
    max_index
}
#[cfg(test)]
mod max_test {
    use crate::utils::max;
    #[test]
    fn one_element() {
        let a = [0];
        assert_eq!(max(&a), 0);
    }
    #[test]
    fn two_element() {
        let a = [0, 1];
        assert_eq!(max(&a), 1);
    }
    #[test]
    fn more_element() {
        let a = [0, 1, 0, 2, 9, 9, 20, 0, 0];
        assert_eq!(max(&a), 6);
    }
}
