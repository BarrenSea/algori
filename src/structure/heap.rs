macro_rules! left_child {
    ($parent:ident) => {
	 ($parent <<1) + 1 as usize
     };
}


macro_rules! right_child {
    ($parent:ident) => {
	 ($parent <<1) + 2 as usize
     };
}


macro_rules! parent {
    ($child:ident) => {
	 ($child - 1) >> 1
     };
}




/// # Heap(Priority Queue)
/// Heap is a  completely binary tree , Its parent node always maintains a relationship(define in comparator) with its child nodes
/// # Example
/// ```
/// use algori::structure::Heap;
/// //Create a Max Priority(Max Heap)
/// let mut a: Heap<i32> = Heap::new(|a,b| a >= b);
/// a.push(1); a.push(100); a.push(40); a.push(0);
/// assert_eq!(a.pop().unwrap(),100); assert_eq!(a.pop().unwrap(),40);
/// ```
#[derive(Debug,Clone)]
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T,&T) -> bool,
}

use core::ops::Index;
impl<T> Index<usize> for Heap<T>
{
    type Output = T;
    fn index(&self,index: usize) -> &Self::Output {
	return &self.items[index];
    }
}

#[allow(dead_code)]
impl<T: PartialOrd> Heap<T> {
    ///  Create A Heap With A Comparator
    /// ```
    /// use algori::structure::Heap;
    /// let a: Heap<i32> = Heap::new(|a,b| a >= b);
    /// ```
    pub fn new(comparator: fn(&T,&T) -> bool)  -> Self {
	return Self {
	items: vec!(),
	comparator,
	};
    }
    /// Return The Length Of Heap
    pub fn len(&self) -> usize {
	return self.items.len();
    }
    pub fn is_empty(&self) -> bool {
	return self.items.is_empty();
    }
    /// heapify_down use the comparator
    fn heapify_down(&mut self,start: usize,end:usize) {
    	let mut better_element_index: usize = start;
	if right_child!(start) < end  && (self.comparator)(&self.items[right_child!(start)],&self.items[better_element_index]) {
	    better_element_index = right_child!(start);
	}
	if left_child!(start) < end  &&  (self.comparator)(&self.items[left_child!(start)],&self.items[better_element_index]) {
	    better_element_index = left_child!(start);
	}
	if better_element_index != start {
	    self.items.swap(start,better_element_index);
	    self.heapify_down(better_element_index,end);
	}
    }
    /// Push A New Element and heapify
    pub fn push(&mut self,value: T) ->()
    {
	self.items.push(value);
	let mut point: usize = self.len() - 1;
	while point > 0 && (self.comparator)(&self.items[point],&self.items[parent!(point)]) {
	    self.items.swap(point,parent!(point));
	    point = parent!(point);
	}
    }
    /// Pop the Top Element
    pub fn pop(&mut self) -> Result<T,&str> {
	if self.is_empty() {return Err("Heap Is Empty!");}
	if self.len() == 1 {return Ok(self.items.pop().unwrap());}
	// pop the top element
	let pop_element:T = self.items.swap_remove(0);
	self.heapify_down(0,self.len());
	Ok(pop_element)
    }
    /// heap_sort use the comparator
    pub fn sort(&mut self) -> () {
	let len = self.len();
	// heap_sort
	for i in (1..len).rev() {
            self.items.swap(0, i);
            self.heapify_down(0, i);
	}
    }

    /// clear all the elements
    pub fn clear(&mut self) ->() {
	self.items.clear();
    }
}

#[allow(unused_parens)]
#[allow(dead_code)]
impl<T> Heap<T> {
    pub fn comparator(&self) -> (fn(&T,&T) -> bool)
    {
	return self.comparator;
    }
}




#[cfg(test)]
mod heap_tests {
    use super::Heap;
    #[test]
    fn comparator_test() ->() {
	let a: Heap<i32> = Heap::new(|a,b| a > b);
	let c = (a.comparator())(&6,&7);
	assert_eq!(c,false)
    }
    #[test]
    fn macro_test() ->() {
	let mut a  = 0;
	let l = left_child!(a);
	let r = right_child!(a);
	assert_eq!(l,1);
	assert_eq!(r,2);
	a = 20;
	let p = parent!(a);
	assert_eq!(9,p);
    }
    #[test]
    fn max_heap_operation() -> () {
	let mut a: Heap<i32>  = Heap::new(|a,b| a >= b);
	a.push(60);
	a.push(2);
	a.push(30);
	a.push(100);
	let mut vec: Vec<i32> = vec!();
	vec.push(100);
	vec.push(60);
	vec.push(30);
	vec.push(2);
	assert_eq!(a.items,vec);
    }
    #[test]
    fn min_heap_operation() -> (){
	let mut a: Heap<i32>  = Heap::new(|a,b| a <= b);
	a.push(60);
	a.push(2);
	a.push(30);
	a.push(100);
	a.push(1);
	a.push(40);
	let mut vec: Vec<i32> = vec!();
	vec.push(1);
	vec.push(2);
	vec.push(30);
	vec.push(100);
	vec.push(60);
	vec.push(40);
	assert_eq!(a.items,vec);
    }
    #[test]
    fn sort() -> () {
	let mut a: Heap<i32> = Heap::new(|a,b| a>=b);
	a.push(199); a.push(0); a.push(20); a.push(40);
	a.sort();
	assert_eq!(a.items,[0,20,40,199]);
    }
}
