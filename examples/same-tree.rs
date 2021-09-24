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
            let (left, right, expected) = $value;
            assert_eq!(expected, is_same_tree(left, right));
        }
    )*
    }
}

tests! {
    example_1: (Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))), Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))), true),
    example_2: (Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))), Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))), false),
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(x), Some(y)) => {
            let left = x.borrow();
            let right = y.borrow();
            left.val == right.val
                && is_same_tree(left.left.clone(), right.left.clone())
                && is_same_tree(left.right.clone(), right.right.clone())
        }
    }
}

fn main() {
    let first_left = TreeNode {
        val: 2,
        left: None,
        right: None,
    };
    let first_right = TreeNode {
        val: 3,
        left: None,
        right: None,
    };
    let first = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(first_left))),
        right: Some(Rc::new(RefCell::new(first_right))),
    };
    println!(
        "{:?}",
        is_same_tree(
            Some(Rc::new(RefCell::new(first.clone()))),
            Some(Rc::new(RefCell::new(first.clone())))
        )
    );
}
