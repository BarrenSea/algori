/// 链表节点
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// 链表
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    /// 创建一个新的空链表
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// 在链表头部插入一个新节点
    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value: value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    /// 移除并返回链表头部的节点
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    /// 返回链表头部节点的引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// 检查链表是否为空
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    ///链表搜索,返回指针
    pub fn search(&self, value: &T) -> Option<&Node<T>> {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.value == value {
                return Some(node);
            }
            current = &node.next;
        }
        None
    }
}

