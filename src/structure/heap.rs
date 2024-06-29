use core::ops::{Deref, DerefMut};
use std::fmt::Display;

macro_rules! left_child {
    ($parent:ident) => {
        ($parent << 1) + 1 as usize
    };
}

macro_rules! right_child {
    ($parent:ident) => {
        ($parent << 1) + 2 as usize
    };
}

macro_rules! parent {
    ($child:ident) => {
        ($child - 1) >> 1
    };
}

/// # Heap(Priority Queue)
/// Heap is a **completely binary tree** , The father  is not less than the son  (big root pile). Similarly, we can define a small root heap.
///
/// - if comparator is ``` |a,b| a >= b ```, the heap will be a *MaxPriorityQueue* or *max root heap*.
///
/// - if comparator is ``` |a,b| a <= b ```, the heap will be a *MinPriorityQueue* or *small root heap*.
/// # Example
/// ```
/// use algori::structure::Heap;
/// //Create a Max Priority(Max Heap)
/// let mut a: Heap<i32> = Heap::new(|a,b| a >= b);
/// a.push(1); a.push(100); a.push(40); a.push(0);
/// assert_eq!(a.pop().unwrap(),100); assert_eq!(a.pop().unwrap(),40);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

/// # DeRef the Heap
/// # Example
/// ```
/// use algori::structure::Heap;
/// let mut a: Heap<i32> = Heap::new(|a,b| a >= b);
/// a.push(1); a.push(100); a.push(0); a.push(50);
/// assert_eq!(*a,[100,50,0,1]);
/// assert_eq!((*a)[0],100);
/// fn ps(a: &[i32])->(){};
/// ps(&a);
/// ```
impl<T> Deref for Heap<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        return &self.items;
    }
}

impl<T> DerefMut for Heap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.items;
    }
}

impl<T> Default for Heap<T>
where
    T: PartialOrd,
{
    fn default() -> Self {
        return Heap {
            items: Vec::default(),
            comparator: |a, b| a >= b,
        };
    }
}
impl<T> Display for Heap<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.items);
    }
}
/// from a [T;N]
impl<T: PartialOrd, const N: usize> From<[T; N]> for Heap<T> {
    fn from(array: [T; N]) -> Self {
        let mut heap = Heap {
            items: array.into(),
            comparator: |a, b| a >= b,
        };
        heap.rebuild();
        return heap;
    }
}
/// from Vec<T>
impl<T: PartialOrd> From<Vec<T>> for Heap<T> {
    fn from(array: Vec<T>) -> Self {
        let mut heap = Heap {
            items: array,
            comparator: |a, b| a >= b,
        };
        heap.rebuild();
        return heap;
    }
}
#[allow(dead_code)]
impl<T: PartialOrd> Heap<T> {
    /// # Create a Heap With a Comparator
    /// ```
    /// use algori::structure::Heap;
    /// let a: Heap<i32> = Heap::new(|a,b| a >= b);
    /// ```
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        return Self {
            items: vec![],
            comparator,
        };
    }

    /// # Create a Heap With a Comparator and a vec
    /// ```
    /// use algori::structure::Heap;
    /// let a: Heap<i32> = Heap::new_from(vec!(1,2,3,4,5,6,7),|a,b| a >= b);
    /// ```
    pub fn new_from(vec: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        let mut heap = Heap {
            items: vec,
            comparator,
        };
        heap.rebuild();
        return heap;
    }
    /// # Get the Borror of the first element
    /// ```
    /// use algori::structure::Heap;
    /// let a: Heap<i32> = Heap::new_from(vec!(1,2,3,4,5,6,7),|a,b| a >= b);
    /// assert_eq!(a.peek().unwrap(),&7);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return Some(&self[0]);
    }

    /// # Get the Vec of the Heap
    pub fn into_vec(self) -> Vec<T> {
        return self.items;
    }
    /// Return The Length Of Heap
    pub fn len(&self) -> usize {
        return self.items.len();
    }
    /// Check If the Heap is empty
    pub fn is_empty(&self) -> bool {
        return self.items.is_empty();
    }
    /// heapify_down use the comparator
    fn heapify_down(&mut self, start: usize, end: usize) {
        let mut better_element_index: usize = start;
        if right_child!(start) <= end
            && (self.comparator)(
                &self.items[right_child!(start)],
                &self.items[better_element_index],
            )
        {
            better_element_index = right_child!(start);
        }
        if left_child!(start) <= end
            && (self.comparator)(
                &self.items[left_child!(start)],
                &self.items[better_element_index],
            )
        {
            better_element_index = left_child!(start);
        }
        if better_element_index != start {
            self.items.swap(start, better_element_index);
            self.heapify_down(better_element_index, end);
        }
    }
    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            self.heapify_down(n, self.len() - 1);
        }
    }
    /// Push A New Element and heapify
    pub fn push(&mut self, value: T) -> () {
        self.items.push(value);
        let mut point: usize = self.len() - 1;
        while point > 0 && (self.comparator)(&self.items[point], &self.items[parent!(point)]) {
            self.items.swap(point, parent!(point));
            point = parent!(point);
        }
    }
    /// Pop the Top Element
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        if self.len() == 1 {
            return Some(self.items.pop().unwrap());
        }
        // pop the top element
        let pop_element: T = self.items.swap_remove(0);
        self.heapify_down(0, self.len() - 1);
        return Some(pop_element);
    }
    /// heap_sort use the comparator
    /// The bad time is O( n log2 n)
    /// # Example
    /// ```
    /// use algori::structure::Heap;
    /// // Create a Max Priority(Max Heap)
    /// let mut a: Heap<i32> = Heap::new(|a,b| a >= b);
    /// a.push(1); a.push(100); a.push(40); a.push(0);
    /// a.sort();
    /// assert_eq!(&*a,&[0,1,40,100]);
    /// ```
    pub fn sort(&mut self) -> () {
        let len = self.len();
        // heap_sort
        for i in (1..len).rev() {
            self.items.swap(0, i);
            self.heapify_down(0, i - 1);
        }
    }

    /// clear all the elements
    pub fn clear(&mut self) -> () {
        self.items.clear();
    }

    /// get the comparator of the Heap
    /// # Example
    /// ```
    /// use algori::structure::Heap;
    /// // Create a Max Priority(Max Heap)
    /// let mut a: Heap<i32> = Heap::new(|a,b| a >= b);
    /// let comparator: (fn(&i32,&i32) -> bool) = a.comparator();
    /// assert_eq!(comparator(&10,&1),true);
    /// assert_eq!(comparator(&100,&1000),false);
    /// ```
    #[allow(unused_parens)]
    pub fn comparator(&self) -> (fn(&T, &T) -> bool) {
        return self.comparator;
    }

    /// 以任意顺序返回底层 vector 中所有值的切片。
    ///
    ///
    /// # Examples
    ///
    /// 基本用法：
    ///
    /// ```
    /// use algori::structure::Heap;
    /// let mut a = Heap::new(|a,b|a>=b);
    /// a.push(1); a.push(2); a.push(100);
    /// assert_eq!(a.as_slice(),&[100,1,2]);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        return &self.items;
    }
}

#[cfg(test)]
mod heap_tests {
    use super::super::super::sorting::is_sorted;
    use super::Heap;
    #[test]
    fn comparator_test() -> () {
        let a: Heap<i32> = Heap::new(|a, b| a > b);
        let c = (a.comparator())(&6, &7);
        assert_eq!(c, false)
    }
    #[test]
    fn macro_test() -> () {
        let mut a = 0;
        let l = left_child!(a);
        let r = right_child!(a);
        assert_eq!(l, 1);
        assert_eq!(r, 2);
        a = 20;
        let p = parent!(a);
        assert_eq!(9, p);
    }
    #[test]
    fn max_heap_operation() -> () {
        let mut a: Heap<i32> = Heap::new(|a, b| a >= b);
        a.push(60);
        a.push(2);
        a.push(30);
        a.push(100);
        let mut vec: Vec<i32> = vec![];
        vec.push(100);
        vec.push(60);
        vec.push(30);
        vec.push(2);
        assert_eq!(a.items, vec);
    }
    #[test]
    fn min_heap_operation() -> () {
        let mut a: Heap<i32> = Heap::new(|a, b| a <= b);
        a.push(60);
        a.push(2);
        a.push(30);
        a.push(100);
        a.push(1);
        a.push(40);
        let mut vec: Vec<i32> = vec![];
        vec.push(1);
        vec.push(2);
        vec.push(30);
        vec.push(100);
        vec.push(60);
        vec.push(40);
        assert_eq!(a.items, vec);
    }
    #[test]
    fn pop() -> () {
        let mut a: Heap<i32> = Heap::new(|a, b| a >= b);
        a.push(199);
        a.push(0);
        a.push(20);
        a.push(40);
        a.push(2000);
        assert_eq!(a.pop().unwrap(), 2000);
        let mut b = a.clone();
        b.sort();
        assert!(is_sorted(&b, |a, b| a <= b));
        assert_eq!(a.pop().unwrap(), 199);
        let mut b = a.clone();
        b.sort();
        assert!(is_sorted(&b, |a, b| a <= b));
    }

    #[test]
    fn sort() -> () {
        let mut a: Heap<i32> = Heap::new(|a, b| a >= b);
        a.push(199);
        a.push(0);
        a.push(20);
        a.push(40);
        a.sort();
        assert_eq!(a.items, [0, 20, 40, 199]);
    }
}
