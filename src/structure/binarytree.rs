
/// # Examples
/// ```
///use algori::structure::BinaryTree;
///fn test_add_method_1() {
///    let planets = vec!["Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus"];
///    let mut tree = BinaryTree::Empty;
///    for planet in planets {
///        tree.add(planet);
///    }
///
///    assert_eq!(tree.walk(),
///               vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
///}
/// ```
///二叉树
///来自于Oreilly 《Rust程序设计》
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

///创建树的节点
impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}    

impl<T: Clone> BinaryTree<T> {
    pub fn walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.walk();
                result.push(boxed.element.clone());
                result.extend(boxed.right.walk());
                result
            }
        }
    }
}
