use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod test {
    use super::*;
    use crate::btree;

    #[test]
    fn test_1() {
        assert_eq!(is_valid_bst(btree![2, 1, 3]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid_bst(btree![5, 1, 3]), false);
    }
}

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

pub fn is_valid_bst(root: MaybeNode) -> bool {
    fn helper(node: &MaybeNode, possible_min: i64, possible_max: i64) -> bool {
        if let Some(n) = node {
            let borrowed = n.borrow();
            let left = &borrowed.left;
            let right = &borrowed.right;
            let val: i64 = borrowed.val.into();
            if val >= possible_min && val <= possible_max {
                helper(&left, possible_min, val) && helper(&right, val, possible_max)
            } else {
                false
            }
        } else {
            true
        }
    }
    helper(&root, i64::MIN, i64::MAX)
}
