use std::{fmt::Display, ptr::NonNull};

// 节点
#[derive(Debug)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

#[derive(Debug)]
/// LinkedList(双向)
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    // marker表示本结构有一个Box<Node<T>>的所有权
    // 链表的节点申请和释放都是通过Box<T>完成 所以拥有所有权
    marker: core::marker::PhantomData<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        return Node {
            next: None,
            prev: None,
            element,
        };
    }
    fn into_element(self) -> T {
        return self.element; // 消费Box后 堆内存释放 并复制element到栈
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<T> LinkedList<T> {
    /// 构造空的双向链表
    pub const fn new() -> Self {
        return LinkedList {
            head: None,
            tail: None,
            len: 0usize,
            marker: core::marker::PhantomData,
        };
    }
    /// 返回尾的引用
    pub fn back(&self) -> Option<&T> {
        if let Some(tail) = self.tail {
            unsafe {
                return Some(&(*tail.as_ptr()).element);
            }
        } else {
            // 尾为空
            return None;
        }
    }
    /// 返回头的引用
    pub fn front(&self) -> Option<&T> {
        if let Some(head) = self.head {
            unsafe {
                return Some(&(*head.as_ptr()).element);
            }
        } else {
            // 头为空
            return None;
        }
    }

    /// 在链表尾部追加节点
    pub fn push_back(&mut self, element: T) {
        let mut new_node = Box::new(Node::new(element));
        // 新节点的prev指向链表的尾部
        new_node.prev = self.tail;
        // 转换为NonNull指针
        let new_node = NonNull::new(Box::into_raw(new_node));
        match self.tail {
            Some(node) => {
                // 旧尾部需要解引用裸指针
                unsafe {
                    // 链表尾部的next指向新节点
                    (*node.as_ptr()).next = new_node;
                }
            }
            // 链表头部指向新节点
            None => self.head = new_node,
        }
        // 链表尾部指向新节点
        self.tail = new_node;
        self.len += 1;
    }
    /// 在链表尾部追加节点
    pub fn push_front(&mut self, element: T) {
        let mut new_node = Box::new(Node::new(element));
        // 新节点的next指向链表的头部
        new_node.next = self.head;
        // 转换为NonNull指针
        let new_node = NonNull::new(Box::into_raw(new_node));
        match self.head {
            Some(node) => {
                // 旧头部需要解引用裸指针
                unsafe {
                    // 链表尾部的next指向新节点
                    (*node.as_ptr()).prev = new_node;
                }
            }
            // 链表尾部指向新节点
            None => self.tail = new_node,
        }
        // 链表头部指向新节点
        self.head = new_node;
        self.len += 1;
    }
    /// 返回链表长度
    pub fn len(&self) -> usize {
        return self.len;
    }

    /// 根据给出的index(节点的序号) 插入到index
    /// ```
    /// use algori::structure::LinkedList;
    /// let mut a = LinkedList::new();
    /// a.push_back(0);
    /// a.push_back(0);
    /// a.push_back(1);
    /// a.push_front(2);
    /// a.insert(1,9);
    /// a.insert(3,10);
    ///
    /// ```
    ///
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), String> {
        // 索引越界
        if self.len < index {
            return Err(format!(
                "LinkedList len is {}, but index is {}",
                self.len, index
            ));
        }
        // 链表为空 或 索引为0,插入开头
        if index == 0 || self.head.is_none() {
            self.push_front(element);
            return Ok(());
        }
        // 索引在尾,插入结尾
        if self.len == index {
            self.push_back(element);
            return Ok(());
        }
        // 遍历到插入点
        if let Some(mut current_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*current_node.as_ptr()).next {
                        None => {
                            return Err(format!(
                                "LinkedList len is {}, but index is {}",
                                self.len, index
                            ))
                        }
                        Some(next_ptr) => current_node = next_ptr,
                    }
                }
            }
            // 创建新节点
            let mut new_node = Box::new(Node::new(element));
            unsafe {
                new_node.prev = (*current_node.as_ptr()).prev;
                new_node.next = Some(current_node);
            }
            unsafe {
                if let Some(old_prev) = (*current_node.as_ptr()).prev {
                    let node_ptr = NonNull::new(Box::into_raw(new_node));
                    // 更改插入点前面节点的next
                    (*old_prev.as_ptr()).next = node_ptr;
                    // 更改插入点前面节点的prev
                    (*current_node.as_ptr()).prev = node_ptr;
                    self.len += 1;
                }
            }
        }

        Ok(())
    }
    /// 弹出链表头部元素 若头部为None则返回None
    /// ```
    /// use algori::structure::LinkedList;
    /// let mut a = LinkedList::new();
    /// a.push_back(0);
    /// a.push_back(0);
    /// a.push_back(1);
    /// a.push_front(2);
    /// a.insert(1,9);
    /// a.insert(3,10);
    /// assert_eq!(a.pop_front(),Some(2));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // 获取头部元素
        if let Some(node) = self.head {
            unsafe {
                // 旧头的next
                if let Some(new_head) = (*node.as_ptr()).next {
                    // 新头的prev变成None
                    (*new_head.as_ptr()).prev = None;
                    // 旧头获得所有权
                    let node = Box::from_raw(node.as_ptr());
                    // head指向旧头的next
                    self.head = Some(new_head);
                    // 返回旧头的元素
                    return Some(node.into_element());
                } else {
                    // 当头的next为None时
                    // 将头也设置为None
                    if let Some(head) = self.head {
                        self.head = None;
                        let node = Box::from_raw(head.as_ptr());
                        return Some(node.into_element());
                    }
                }
            }
            // 头部为空 返回None
        }
        return None;
    }
    /// 弹出链表尾部元素 若头部为None则返回None
    /// ```
    /// use algori::structure::LinkedList;
    /// let mut a = LinkedList::new();
    /// a.push_back(0);
    /// a.push_back(0);
    /// a.push_back(1);
    /// a.push_front(2);
    /// a.push_back(9);
    /// a.insert(1,9);
    /// a.insert(3,10);
    /// assert_eq!(a.pop_back(),Some(9));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // 获取尾部元素
        if let Some(node) = self.tail {
            unsafe {
                // 旧尾的prev
                if let Some(new_tail) = (*node.as_ptr()).prev {
                    // 新尾的next变成None
                    (*new_tail.as_ptr()).next = None;
                    // 旧尾获得所有权
                    let node = Box::from_raw(node.as_ptr());
                    // tail指向旧尾的prev
                    self.tail = Some(new_tail);
                    // 返回旧尾的元素
                    return Some(node.into_element());
                } else {
                    // 当尾的next为None时
                    // 将尾也设置为None
                    if let Some(tail) = self.tail {
                        self.tail = None;
                        let node = Box::from_raw(tail.as_ptr());
                        return Some(node.into_element());
                    }
                }
            }
            // 尾部为空 返回None
        }
        return None;
    }
    /// 弹出链表位于index的元素
    /// ```
    /// use algori::structure::LinkedList;
    /// let mut a = LinkedList::new();
    /// a.push_back(0);
    /// a.push_back(0);
    /// a.push_back(1);
    /// a.push_front(2);
    /// a.insert(1,9);
    /// a.insert(3,10);
    /// a.insert(4,232);
    /// assert_eq!(Some(232),a.pop(4));
    /// ```
    ///
    pub fn pop(&mut self, index: usize) -> Option<T> {
        // 索引越界
        if self.len < index {
            return None;
        }
        // 链表为空 或 索引为0,插入开头
        if index == 0 || self.head.is_none() {
            return self.pop_front();
        }
        // 索引在尾,插入结尾
        if self.len == index {
            return self.pop_back();
        }

        if let Some(mut current_node) = self.head {
            // 遍历到插入点
            for _ in 0..index {
                unsafe {
                    match (*current_node.as_ptr()).next {
                        None => {
                            return None;
                        }
                        Some(next_ptr) => current_node = next_ptr,
                    }
                }
            }
            unsafe {
                // 插入点前一个节点
                if let Some(prev_node) = (*current_node.as_ptr()).prev {
                    // 插入点后一个节点
                    if let Some(next_node) = (*current_node.as_ptr()).next {
                        // 前一个节点的next指向后一个节点的prev
                        (*prev_node.as_ptr()).next = Some(next_node);
                        // 后一个节点的prev指向前一个节点的next
                        (*next_node.as_ptr()).prev = Some(prev_node);
                    }
                }
            }
            // 获得删除节点所有权
            unsafe {
                let delete_node = Box::from_raw(current_node.as_ptr());
                return Some(delete_node.into_element());
            }
        }

        None
    }
    /// 返回index的节点元素的引用
    /// ```
    /// use algori::structure::LinkedList;
    /// let mut a = LinkedList::new();
    /// a.push_back(0);
    /// a.push_back(0);
    /// a.push_back(1);
    /// a.push_front(2);
    /// a.insert(1,9);
    /// a.insert(3,10);
    /// a.insert(4,232);
    /// assert_eq!(Some(&232),a.get(4));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        // 索引越界
        if self.len < index || self.head.is_none() {
            return None;
        }
        // 头
        if index == 0 {
            if let Some(element) = self.head {
                unsafe {
                    return Some(&(*element.as_ptr()).element);
                }
            }
        }
        if let Some(mut current_node) = self.head {
            // 遍历到点
            for _ in 0..index {
                unsafe {
                    match (*current_node.as_ptr()).next {
                        None => {
                            return None;
                        }
                        Some(next_ptr) => current_node = next_ptr,
                    }
                }
            }
            unsafe {
                return Some(&(*current_node.as_ptr()).element);
            }
        }
        None
    }
    /// 消耗linkedlist变成vec
    pub fn to_vec(mut self) -> Vec<T> {
        let mut vec = vec![];
        while self.head.is_some() {
            if let Some(value) = self.pop_front() {
                vec.push(value);
            }
        }
        return vec;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len == 0 {
            return Ok(());
        }
        if let Some(mut current_node) = self.head {
            unsafe {
                write!(f, "{}", (*current_node.as_ptr()).element).unwrap();
            }
            for _ in 0..self.len {
                unsafe {
                    match (*current_node.as_ptr()).next {
                        Some(node) => {
                            write!(f, "->{}", (*node.as_ptr()).element).unwrap();
                            current_node = node;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
        return Ok(());
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop items until there are none left
        while self.pop_front().is_some() {}
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(array: [T; N]) -> Self {
        let mut new_linkedlist = LinkedList::new();
        for value in array {
            new_linkedlist.push_back(value);
        }
        return new_linkedlist;
    }
}

/// 将两个链表相加 每个节点为0-9
/// ```
/// use algori::structure::LinkedList;
/// use algori::structure::linkedlist::add_two_linkedlist;
/// let a: LinkedList<i32> = [1,3,2,5,5,2].into();
/// let b: LinkedList<i32> = [2,3,1,9,1,4,6,8].into();
/// assert_eq!(&add_two_linkedlist(a,b).to_vec(),&[3,6,3,4,7,6,6,8]);
/// ```
pub fn add_two_linkedlist(a: LinkedList<i32>, b: LinkedList<i32>) -> LinkedList<i32> {
    let mut result = LinkedList::new();
    let mut current = &mut result;
    let (mut p1, mut p2) = (a, b);
    let mut sum = 0i32;
    while p1.front().is_some() || p2.front().is_some() || sum != 0 {
        if let Some(value) = p1.pop_front() {
            sum += value;
        }
        if let Some(value) = p2.pop_front() {
            sum += value;
        }
        current.push_back(sum % 10);
        sum = sum / 10;
    }
    return result;
}

/// 将两个链表相加 每个节点为二进制
/// ```
/// use algori::structure::LinkedList;
/// use algori::structure::linkedlist::add_two_binary_linkedlist;
/// let a: LinkedList<bool> = [true,false,true,false,false,false].into();
/// let b: LinkedList<bool> = [true,false,false,false,true].into();
/// assert_eq!(&add_two_binary_linkedlist(a,b).to_vec(),&[false,true,true,false,true,false]);
/// ```
pub fn add_two_binary_linkedlist(a: LinkedList<bool>, b: LinkedList<bool>) -> LinkedList<bool> {
    let mut result = LinkedList::new();
    let (mut p1, mut p2) = (a, b);
    let mut carry = false;
    while p1.front().is_some() || p2.front().is_some() || carry {
        let (value1, value2) = match (p1.pop_front(), p2.pop_front()) {
            (Some(v1), Some(v2)) => (v1, v2),
            (Some(v1), None) => (v1, false),
            (None, Some(v2)) => (false, v2),
            (None, None) => {
                // 当两个链表都为空，只有进位时
                if carry {
                    carry = false;
                    (true, false)
                } else {
                    break;
                }
            }
        };
        let xor1 = value1 ^ value2;
        // 利用全加器
        let sum = xor1 ^ carry; // 三路异或 判断奇数
        carry = (xor1 & carry) | (value1 & value2); // 三路与 计算偶数进位
        result.push_back(sum);
    }

    return result;
}

#[cfg(test)]
mod add_two_binary_test {
    use super::add_two_binary_linkedlist;
    use super::LinkedList;
    #[test]
    fn empty() -> () {
        let a: LinkedList<bool> = [].into();
        let b: LinkedList<bool> = [].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[]);
    }
    #[test]
    fn carry() -> () {
        let a: LinkedList<bool> = [true].into();
        let b: LinkedList<bool> = [true].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[false, true]);
    }
    #[test]
    fn one() -> () {
        let a: LinkedList<bool> = [false].into();
        let b: LinkedList<bool> = [true].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[true]);
    }
    #[test]
    fn two() -> () {
        let a: LinkedList<bool> = [true, false].into();
        let b: LinkedList<bool> = [true].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[false, true]);
    }
    #[test]
    fn more() -> () {
        let a: LinkedList<bool> = [false, false].into();
        let b: LinkedList<bool> = [true].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[true, false]);
    }
    #[test]
    fn common() -> () {
        let a: LinkedList<bool> = [false, false, true, true, false, true].into();
        let b: LinkedList<bool> = [true, false, true, false, true, false, true, true].into();
        let result = add_two_binary_linkedlist(a, b);
        assert_eq!(
            &result.to_vec(),
            &[true, false, false, false, false, false, false, false, true]
        );
    }
}

mod add_two_linkedlist {
    use super::add_two_linkedlist;
    use super::LinkedList;
    #[test]
    fn empty() -> () {
        let a: LinkedList<i32> = [].into();
        let b: LinkedList<i32> = [].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[]);
    }
    #[test]
    fn carry() -> () {
        let a: LinkedList<i32> = [9].into();
        let b: LinkedList<i32> = [8].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[7, 1]);
    }
    #[test]
    fn one() -> () {
        let a: LinkedList<i32> = [2].into();
        let b: LinkedList<i32> = [3].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[5]);
    }
    #[test]
    fn two() -> () {
        let a: LinkedList<i32> = [9, 2].into();
        let b: LinkedList<i32> = [3, 9].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[2, 2, 1]);
    }
    #[test]
    fn more() -> () {
        let a: LinkedList<i32> = [9, 2, 9, 2].into();
        let b: LinkedList<i32> = [3, 9].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(&result.to_vec(), &[2, 2, 0, 3]);
    }
    #[test]
    fn common() -> () {
        let a: LinkedList<i32> =
            [0, 2, 3, 1, 5, 3, 6, 2, 5, 8, 1, 8, 9, 2, 9, 5, 3, 5, 9, 0].into();
        let b: LinkedList<i32> =
            [8, 2, 3, 6, 2, 1, 6, 2, 9, 1, 5, 1, 6, 5, 8, 2, 3, 1, 5, 2].into();
        let result = add_two_linkedlist(a, b);
        assert_eq!(
            &result.to_vec(),
            &[8, 4, 6, 7, 7, 4, 2, 5, 4, 0, 7, 9, 5, 8, 7, 8, 6, 6, 4, 3]
        );
    }
}
