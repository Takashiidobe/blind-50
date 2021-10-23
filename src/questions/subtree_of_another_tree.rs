use crate::*;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

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

test! {
    test_1: is_subtree(btree![3, 4, 5, 1, 2], btree![4, 1, 2]), true,
    test_2: is_subtree(btree![1, 2, 3, 4], btree![1, 2, 3, 5]), false,
}
