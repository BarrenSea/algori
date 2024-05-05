///二分插入排序
///# Examples
///```
///
///let mut  a  = [7,3,5,1,9,65,65,4,6,6];
///
///use algori::sort::binary_sort;
///let c = binary_sort(&mut a);
///assert_eq!(a,[1,3,4,5,6,6,7,9,65,65]);
///```
pub async fn sort<T>(array: &mut[T]) ->() where
    T: PartialOrd, {
    	for i in 1..array.len() {
	    let mut q = i;
	    let ptr = crate::search::binary_search(&array[0..i],&array[i]).await;
	    match ptr {
	    Ok(a) => {
		while q > a {
		    array.swap(q,q-1);
		    q -= 1;
		  
		};
	    },
	    Err(mut a) => {
		while q > a {
		    array.swap(q,q-1);
		    q -= 1;
		};
	    },
	};

	}
}

