///排序算法
pub mod sort;

///矩阵算法
pub mod matrix;
///子数组算法
pub mod subarray;

///查找算法
pub mod search;


///数据结构
pub mod structure;


///数学算法与变换
pub mod math;
#[cfg(test)]
mod tests {
    use crate::structure::LinkedList;
    use crate::structure::LinkedListNode;
    #[test]
    fn test() {
        let mut a = [1,2,3,0,5,7,9,8,32,299,100];
        crate::sort::insertion_sort(&mut a);
	assert_eq!(a,[0,1,2,3,5,7,8,9,32,100,299]);
        let m = (3,[1,2,3,4,5,6,7,8,9]);
        let n = crate::matrix::square_matrix_multiply(m,m);
        assert_eq!(n,(3, [30, 36, 42, 66, 81, 96, 102, 126, 150]));

        let q = [-3,1,2,-8,9,0,2,-10,-5,-20,40,-90];
        let c = crate::subarray::merge_max_subarray(&q);
	assert_eq!(c,(4, 10, 16));
        let a = [0,1,3,6,124,232,1312312];
        let c = crate::search::binary_search(&a,&7);
        assert_eq!(c,Err(4));

        let mut q = [1231,123,1234,125,3,3,56,745,856,0,4,867,1237,4564124,123];
        crate::sort::heap_max_sort(&mut q);
	assert_eq!(q,[0, 3, 3, 4, 56, 123, 123, 125, 745, 856, 867, 1231, 1234, 1237, 4564124]);

        let mut q = [1231,123,1234,125,3,3,56,745,856,0,4,867,1237,4564124,123];
        crate::sort::heap_min_sort(&mut q);
	assert_eq!(q, [4564124, 1237, 1234, 1231, 867, 856, 745, 125, 123, 123, 56, 4, 3, 3, 0]);

        let mut  a  = crate::structure::MaxPriorityQueue::new();
        a.push(1);
        a.push(8);
        a.push(2);
        a.push(4);
        a.push(5);
        assert_eq!(a.pop(),Some(8));
	assert_eq!(a.pop(),Some(5));
	assert_eq!(a.pop(),Some(4));
	assert_eq!(a.pop(),Some(2));
	assert_eq!(a.pop(),Some(1));
	
        let mut a = [1,2,3,0,5,7,9,8,32,299,100];
        crate::sort::quicksort(&mut a);	
	assert_eq!(a,[0,1,2,3,5,7,8,9,32,100,299]);

        let mut a = [1,2,3,0,5,7,9,8,32,299,100];
        crate::sort::count_sort(&mut a);
	assert_eq!(a,[0,1,2,3,5,7,8,9,32,100,299]);

        let mut a = [1,2,3,0,5,7,9,8,32,299,100];
        crate::sort::radix_sort(&mut a);
	assert_eq!(a,[0,1,2,3,5,7,8,9,32,100,299]);
        
        let mut a = [1,2,3,0,5,7,9,8,32,299,100];
        crate::sort::pdqsort(&mut a);

	assert_eq!(a,[0,1,2,3,5,7,8,9,32,100,299]);
        let mut b = crate::structure::Stack::new();
        b.push(1);
        b.push(2);
	assert_eq!(b.pop(),Some(2));
	assert_eq!(b.pop(),Some(1));


	let a = [1,2,3,0,5,7,9,8,32,299,100];
	let c =crate::search::max_search(&a);

	assert_eq!(c,9);	    
	let c =crate::search::min_search(&a);
	assert_eq!(c,3);

	use crate::structure::Complex;
	let a = Complex{real: 1, imag: 2};
	let b = Complex{real: 2 ,imag: 3};
	let c = a + b;
	assert_eq!(c,Complex{real:3, imag: 5});
	
	let signal = vec![0.0, 1.0, 0.0, 1.0]; // Input signal
	let spectrum = crate::math::dft(&signal);

	for (k, value) in spectrum.iter().enumerate() {
            println!("Frequency {}: {:?}", k, value);
	}	
	
        }
    
}
