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
            assert_eq!(expected, is_valid_bst(input));
        }
    )*
    }
}

tests! {
    ex1: (btree![2,1,3], true),
    ex2: (btree![2147483647], true),
    ex3: (btree![2,2,2], true),
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>, possible_min: i64, possible_max: i64) -> bool {
        match node {
            Some(n) => {
                let borrowed = n.borrow();
                let val: i64 = borrowed.val as i64;
                if val > possible_min && val < possible_max {
                    return helper(borrowed.left.clone(), possible_min, val)
                        && helper(borrowed.right.clone(), val, possible_max);
                } else {
                    return false;
                }
            }
            None => true,
        }
    }
    helper(root, i64::MIN, i64::MAX)
}

fn main() {
    println!("{:?}", is_valid_bst(btree![2, 1, 3]));
    println!("{:?}", is_valid_bst(btree![2147483647]));
    println!("{:?}", is_valid_bst(btree![2, 2, 2]));
}
