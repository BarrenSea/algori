///最大优先序列
pub struct Max_Priority_Queue<T> {
    heap:Vec<T>,
}

impl<T: Ord> Max_Priority_Queue<T> {
    ///创建新序列
    pub fn new() -> Max_Priority_Queue<T> {
	Max_Priority_Queue {heap: vec![]}
    }
    ///压入新元素
    pub fn push(&mut self, value: T) {
	self.heap.push(value);
        let mut i = self.heap.len() - 1;
        while i > 0 && self.heap[(i - 1) / 2] < self.heap[i] {
            self.heap.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }
    ///弹出元素
    pub fn pop(&mut self) -> Option<T>{
	if self.heap.is_empty() {
            return None;
        }
        let result = Some(self.heap.swap_remove(0));
        let mut i = 0;
        while 2 * i + 1 < self.heap.len() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut largest = i;
            if self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < self.heap.len() && self.heap[right] > self.heap[largest] {
                largest = right;
            }
            if largest == i {
                break;
            }
            self.heap.swap(i, largest);
            i = largest;
        }
        result
    }
}



