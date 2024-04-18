///两数之和
///```
///let a  = [2,3,4,1,7];
///let b  = algori::search::two_sum(&a,&11).unwrap();
///assert_eq!(b,(2,4));
///```
pub fn two_sum<T>(array: &[T],target: &T) ->Result<(usize,usize),()>
where T: std::hash::Hash + Eq + std::ops::Sub<Output=T> ,
for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
{
    use std::collections::HashMap;
    let mut map: HashMap<&T,usize> = HashMap::new();
    for (index,num) in array.iter().enumerate() {
	let sub = target - num;
	match map.get(&sub) {
	    Some(&j) => return Ok((j,index)),
	    None => {map.insert(num,index);},
	}
    }
    Err(())
}
