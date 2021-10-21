use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: usize) -> i32 {
    let mut v = vec![];
    fn traverse_until(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>, k: usize) {
        if v.len() == k {
            return;
        }
        match node {
            Some(n) => {
                let borrowed = n.borrow();
                traverse_until(&borrowed.left, v, k);
                v.push(borrowed.val);
                traverse_until(&borrowed.right, v, k);
            }
            None => (),
        }
    }
    traverse_until(&root, &mut v, k);
    v[k - 1]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::btree;

    #[test]
    fn test_1() {
        assert_eq!(kth_smallest(btree![3, 1, 4, null, 2], 1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(kth_smallest(btree![5, 3, 6, 2, 4, null, null, 1], 3), 3);
    }
}
