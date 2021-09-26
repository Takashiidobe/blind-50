use blind_50::btree;
use blind_50::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

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
    ex1: (btree![1, 2, 3], btree![1,2,3], true),
    ex2: (btree![1, 2, 3, 4], btree![1,2,3], false),
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
    println!("{:?}", is_same_tree(btree![1, 2, 3], btree![1, 2, 3]));
    println!("{:?}", is_same_tree(btree![1, 2, 3, 4], btree![1, 2, 3]));
}
