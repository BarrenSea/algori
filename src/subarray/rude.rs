///暴力求解最大子数组
pub fn max<T>(array: &[T]) -> (usize,usize,T) where
    T: std::cmp::PartialEq + std::ops::Add<Output = T> + std::cmp::PartialOrd + std::ops::AddAssign + Copy, {
    let mut max: (usize,usize,T) = (0,0,(*array)[0]);
    for i in 0..(*array).len() - 1 {
	let mut sum: T = (*array)[i] ;
	for j in i + 1..(*array).len() {
	    sum += (*array)[j];
	    if sum > max.2 {
		max.0 = i;
		max.1 = j;
		max.2 = sum;
	    }
	}
    }
    max
}

