
fn main() {
 use algori::sort::quicksort;
 let mut a = [7, 3, 5, 1, 9, 65, 65, 4, 6];
 quicksort(&mut a);
    assert_eq!(a, [1, 3, 4, 5, 6, 7, 9, 65, 65]);
    use algori::structure::LinkedList;
let mut a:LinkedList<i32> = LinkedList::new(); ///创建一个i32的链表
for i in 0..101 { ///插入100个节点
 a.push(i);
}
let b = a.search(&9);
println!("{:?}",b);
}	

