use crate::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[cfg(test)]
mod test {
    use super::*;
    use crate::btree;

    #[test]
    fn test_1() {
        assert_eq!(max_path_sum(btree![1, 2, 3]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_path_sum(btree![-10, 9, 20, null, null, 15, 7]), 42);
    }
}

/// Finds the maximum path sum through a binary tree.
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_so_far = i32::MIN;
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max_so_far: &mut i32) -> i32 {
        match node {
            Some(n) => {
                let val = n.borrow().val;
                let l = max(0, helper(&n.borrow().left, max_so_far));
                let r = max(0, helper(&n.borrow().right, max_so_far));
                *max_so_far = max(*max_so_far, val + l + r);
                val + max(l, r)
            }
            None => 0,
        }
    }
    helper(&root, &mut max_so_far);
    max_so_far
}
