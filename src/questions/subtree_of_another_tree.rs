#[allow(unused_imports)]
use crate::btree;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

macro_rules!  tests {
    ($($name:ident: $value:expr,)*) => {
    #[cfg(test)]
    $(
        #[test]
        fn $name() {
            let (left, right, expected) = $value;
            assert_eq!(expected, is_subtree(left, right));
        }
    )*
    }
}

tests! {
    ex1: (btree![3, 4, 5, 1, 2], btree![4, 1, 2], true),
    ex2: (btree![1, 2, 3, 4], btree![1, 2, 3, 5], false),
}

pub fn is_subtree(root: MaybeNode, sub_root: MaybeNode) -> bool {
    fn is_equal(s: &MaybeNode, t: &MaybeNode) -> bool {
        match s {
            Some(node) => {
                s == t || is_equal(&node.borrow().left, t) || is_equal(&node.borrow().right, t)
            }
            _ => false,
        }
    }

    is_equal(&root, &sub_root)
}
