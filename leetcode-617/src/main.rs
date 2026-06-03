use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(n1), Some(n2)) => {
                n1.borrow_mut().val += n2.borrow().val;
                let n1_left = n1.borrow().left.clone();
                let n2_left = n2.borrow().left.clone();

                let n1_right = n1.borrow().right.clone();
                let n2_right = n2.borrow().right.clone();

                let merged_left = merge_trees(n1_left, n2_left);
                let merged_right = merge_trees(n1_right, n2_right);

                n1.borrow_mut().left = merged_left;
                n1.borrow_mut().right = merged_right;
                Some(n1)
            }
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (None, None) => None,
        }
    }
}
