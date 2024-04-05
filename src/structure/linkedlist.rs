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



use std::ptr::NonNull;
use std::marker::PhantomData;
///双向链表
///来自于TheAlgorithms/Rust
pub struct LinkedList<T>{
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}
struct Node<T> {
    element: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    marker: PhantomData<T>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
	Node{ next: None, prev: None, element, marker: PhantomData}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self{
	Self {
	    len: 0,
	    head: None,
	    tail: None,
	}
    }
    pub fn push_head(&mut self,element: T) {
	let mut node = Box::new(Node::new(element));
	node.prev = None;
	let ptr = NonNull::new(Box::into_raw(node));
	match self.head {
	    None => self.tail = ptr,
	    Some(head_ptr) => unsafe {(*head_ptr.as_ptr()).prev = ptr},
	}
    }
    pub fn push_tail(&mut self,element: T) {
	let mut node = Box::new(Node::new(element));
	node.next = None;
	node.prev = self.tail;
	let ptr = NonNull::new(Box::into_raw(node));
	match self.tail{
	    None => self.head = ptr,
	    Some(tail_ptr) => unsafe {(*tail_ptr.as_ptr()).next = ptr},
	}
    }
    pub fn insert(&mut self,index: usize, element:T ) -> Result<(),&str>{
	if self.len < index {
	    return Err("Index Out Of Bounds!");
	}
	if index == 0 || self.head.is_none() {
	    self.push_head(element);
	    return Ok(());
	}
	if self.len == index {
	    self.push_tail(element);
	    return Ok(());
	}
	if let Some(mut node) = self.head {
	    for _ in 0..index {
		match unsafe{(*node.as_ptr()).next} {
		    None => return Err("index ot of bounds"),
		    Some(next_ptr) => unsafe {node = next_ptr},
		}
	    }
	    let mut new_node = Box::new(Node::new(element));
	    unsafe {new_node.prev = (*node.as_ptr()).prev};
	    new_node.next = Some(node);
	    unsafe {if let Some(p) = (*node.as_ptr()).prev {
		let node_ptr = NonNull::new(Box::into_raw(new_node));
		(*p.as_ptr()).next = node_ptr;
		(*node.as_ptr()).prev = node_ptr;
		self.len += 1;
	    }}
	}
	Ok(())
    }

    pub fn delete_head(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.len = self.len.checked_add_signed(-1).unwrap_or(0);
            old_head.element
        })
        // None
    }
    pub fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                Some(mut prev) => prev.as_mut().next = None,
                None => self.head = None,
            }
            self.tail = old_tail.prev;
            self.len -= 1;
            old_tail.element
        })
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        if self.len < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            return self.delete_head();
        }

        if self.len == index {
            return self.delete_tail();
        }

        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            unsafe {
                let old_ith = Box::from_raw(ith_node.as_ptr());
                if let Some(mut prev) = old_ith.prev {
                    prev.as_mut().next = old_ith.next;
                }
                if let Some(mut next) = old_ith.next {
                    next.as_mut().prev = old_ith.prev;
                }

                self.len -= 1;
                Some(old_ith.element)
            }
        } else {
            None
        }
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        Self::get_node(self.head, index).map(|ptr| unsafe { &(*ptr.as_ptr()).element })
    }

    fn get_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(next_ptr),
                _ => Self::get_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}


