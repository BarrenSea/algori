///最大优先队列
pub struct MaxPriQueue<T: PartialOrd>{
    heap: Vec<T>,
}

impl<T: PartialOrd> MaxPriQueue<T> {
    ///返回一个最大优先队列
    pub fn new() -> Self {
	MaxPriQueue{heap: Vec::new()}
    }
    ///返回最大堆的最大值,不获得所有权
    pub fn max(&self) ->Result<&T,&str> {
	if self.heap.len() < 1 {
	    return Result::Err("heap underflow");
	}
	Ok(&self.heap[0])
    }
    ///弹出第一个元素,获得所有权
    pub fn extract(&mut self) ->Result<T,&str> {
	if self.heap.is_empty() {
	    return Result::Err("heap underflow");
	}
	//弹出第一个元素
	let max = Ok(self.heap.swap_remove(0));
	if !self.heap.is_empty() {
	    crate::sort::build_max(&mut self.heap);
	}
	max
    }
    ///更改下标为index的结点的key
    pub fn increase_key(&mut self,key:T,index: usize) -> Result<(),&str>{
	if key < self.heap[index]{
	    return Err("The key is smaller than old");
	}
	self.heap[index] = key;
	let mut index = index;
	while index > 0 && self.heap[(index - 1) / 2] < self.heap[index] {
	    self.heap.swap(index,(index - 1) / 2);
	    index = (index + 1) / 2 - 1;
	}
	Ok(())
    }
    pub fn insert(&mut self, key: T) ->() {
	self.heap.push(key);
	crate::sort::build_max(&mut self.heap);
    }
}
