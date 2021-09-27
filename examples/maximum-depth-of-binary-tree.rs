use blind_50::btree;
use blind_50::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

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
    ex1: (btree![], 0),
    ex2: (btree![3,9,20,null,null,15,7], 3),
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
    println!("{}", max_depth(btree![]));
    println!("{}", max_depth(btree![1, 2, 3, 4, 5]));
}
