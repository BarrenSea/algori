///栈
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    ///建栈
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }
    ///推入栈
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    ///弹出栈
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    ///返回最后一个元素
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    ///判断是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
