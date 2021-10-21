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
        assert_eq!(max_depth(btree![]), 0)
    }

    #[test]
    fn test_2() {
        assert_eq!(max_depth(btree![1, 2, 3, 4, 5, 6]), 3)
    }

    #[test]
    fn test_3() {
        assert_eq!(max_depth(btree![1, 2, 3, 4, 5, 6, 7, 8]), 4)
    }
}

/// A function that finds the maximum depth of a binary tree.
/// It does this recursively:
/// If the current node is `None`, it returns a depth of 0.
/// For a node that exists, it takes the max depth of the left and right subtree and adds 1 to it.
/// It then returns that value.
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(n) => max(
                traverse(&n.borrow().left) + 1,
                traverse(&n.borrow().right) + 1,
            ),
            None => 0,
        }
    }
    traverse(&root)
}
