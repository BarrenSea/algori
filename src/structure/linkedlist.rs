/// # 单向链表
/// # Examples
/// ```
/// use algori::structure::List;
/// //create a list
/// let mut a  = List::new();
/// a = a.prepend("hi");
/// a = a.prepend("hellow");
/// assert_eq!(a.len(),2);
/// ```
pub enum List<T> {
    Node(T, Box<List<T>>),
    ///最后一个结点
    Nil,
}

impl<T: std::fmt::Display> List<T> {
    /// 创建空链表
    pub fn new() -> List<T> {
        List::Nil
    }
    ///在前面创建一个联表
    pub fn prepend(self, element: T) -> List<T> {
        List::Node(element, Box::new(self))
    }

    /// 返回链表的长度
    pub fn len(&self) -> u32 {
        match self {
            List::Node(_,  tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    /// 输出
    pub fn print(&self) -> String {
        match self {
            List::Node(head, tail) => {
                format!("{}, {}", head, tail.print())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}

