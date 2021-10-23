use crate::*;

/// Calculates if two binary search trees have the same values.
/// In this question, there are four possible cases:
/// 1. Both left and right point to a `None` node. In this case, return true.
/// 2. Either left or right points to a `None` node, but the other has a value. In which case, return false.
/// 3. Both left and right point to a node with a value, but the values are different. return false.
/// 4. Both left and right point to nodes with the same value. Return true.
/// Afterwards
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && same(&left.left, &right.left)
                    && same(&left.right, &right.right)
            }
            (None, None) => true,
            (None, _) | (_, None) => false,
        }
    }
    same(&p, &q)
}

test! {
    test_1: is_same_tree(btree![1,2,3], btree![1,2,3]), true,
    test_2: is_same_tree(btree![1,2,3,4], btree![1,2,3]), false,
}
