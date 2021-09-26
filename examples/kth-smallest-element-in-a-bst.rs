use blind_50::btree;
use blind_50::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, index, expected) = $value;
            assert_eq!(expected, kth_smallest(input, index));
        }
    )*
    }
}

tests! {
    ex1: (btree![3, 1, 4, null, 2], 1, 1),
    ex2: (btree![5, 3, 6, 2, 4, null, null, 1], 3, 3),
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut v = vec![];
    fn traverse_until(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, k: i32) {
        if v.len() == (k as usize) {
            return;
        }
        match node {
            Some(n) => {
                let borrowed = n.borrow();
                traverse_until(borrowed.left.clone(), v, k);
                v.push(borrowed.val);
                traverse_until(borrowed.right.clone(), v, k);
            }
            None => return,
        }
    }
    traverse_until(root, &mut v, k);
    v[(k - 1) as usize]
}

fn main() {
    println!("{:?}", kth_smallest(btree![3, 1, 4, null, 2], 1));
    println!(
        "{:?}",
        kth_smallest(btree![5, 3, 6, 2, 4, null, null, 1], 3)
    );
}
