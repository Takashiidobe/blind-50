use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, max_depth(input));
        }
    )*
    }
}

tests! {
    example_0: (None, 0),
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if node.is_none() {
            return depth;
        }
        let copy = node.clone().unwrap();
        let left = copy.borrow().left.clone();
        let right = copy.borrow().right.clone();
        i32::max(traverse(left, depth + 1), traverse(right, depth + 1))
    }
    traverse(root, 0)
}

fn main() {
    println!("{}", max_depth(None));
}
