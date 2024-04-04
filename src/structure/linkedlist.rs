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


use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone)]
struct Node<T> {
    value: Box<T>,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: Box::new(value),
            prev: None,
            next: None,
        }))
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: u64,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.length += 1;
        self.tail = Some(new_node);
    }

    pub fn pop_head(&mut self) -> Option<Box<T>> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev.take();
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().expect("Something went wrong").into_inner().value
        })
    }
}
