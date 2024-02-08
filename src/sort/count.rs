///计数排序
///最坏时间Θ(k+n) 平均时间Θ(k+n)
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::count_sort;
///let c = count_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```

pub fn count_sort(arr: &mut [i32]) {
    // 寻找数组中的最大值
    let max = *arr.iter().max().unwrap_or(&0);

    // 创建一个长度为 max+1 的计数数组，并初始化为 0
    let mut count = vec![0; (max + 1) as usize];

    // 对原始数组中的每个元素进行计数
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    // 根据计数数组重新排列原始数组
    let mut i = 0;
    for (num, &count) in count.iter().enumerate() {
        for _ in 0..count {
            arr[i] = num as i32;
            i += 1;
        }
    }
}

