///最大不重复子串
pub async fn substring(s: &str) -> usize {
    let s = s.as_bytes();
    let mut max = 0;
    let mut left = 0;
    let mut window = [false;128];
    for (right,&c) in s.iter().enumerate() {
	let c = c as usize;
	while window[c] {
	    window[s[left] as usize] = false;
	    left += 1;
	}
	window[c] = true;
	max = max.max(right - left + 1);
    }
    max
}
