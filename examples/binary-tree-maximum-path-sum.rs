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
            assert_eq!(expected, max_path_sum(input));
        }
    )*
    }
}

tests! {
    ex1: (btree![1, 2, 3], 6),
    ex2: (btree![-10, 9, 20, null, null, 15, 7], 42),
}

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_so_far = i32::MIN;
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max_so_far: &mut i32) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let l = std::cmp::max(0, helper(&n.borrow().left, max_so_far));
            let r = std::cmp::max(0, helper(&n.borrow().right, max_so_far));
            *max_so_far = std::cmp::max(*max_so_far, val + l + r);
            val + std::cmp::max(l, r)
        } else {
            0
        }
    }
    helper(&root, &mut max_so_far);
    max_so_far
}

fn main() {
    println!("{:?}", max_path_sum(btree![1, 2, 3]));
    println!("{:?}", max_path_sum(btree![-10, 9, 20, null, null, 15, 7]));
}
