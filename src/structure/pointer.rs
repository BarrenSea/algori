/// 自定义指针类型
/// # Example
/// ```
///    use algori::structure::Pointer;
///    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
///    //创建指针
///    let mut pointer = Pointer::new(&array);
///
///    assert_eq!(*pointer.value(), 3);
///
/// ```
///
///
///
///
///
///

///指针类型
pub struct Pointer<'a, T> {
    array: &'a [T],
    pub index: usize,
}

impl<'a, T> Pointer<'a, T> {
    /// 创建一个新的指针
    pub fn new(array: &'a [T]) -> Pointer<'a, T> {
        Pointer { array, index: 0 }
    }

    /// 获取指针指向的值
    pub fn value(&self) -> &T {
        &self.array[self.index]
    }

    /// 移动指针
    pub fn move_by(&mut self, offset: usize) {
        self.index += offset;
    }
}

