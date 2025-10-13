use std::{cell::RefCell, env::current_exe, rc::Rc};

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

fn main() {
    let mut root = TreeNode::new(1);
    let current = &mut root;
    current.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut current = current.right.as_ref().unwrap().borrow_mut();
    current.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!(
        "{:?}",
        inorder_traversal(Some(Rc::new(RefCell::new(current))))
    );
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res_vector: Vec<i32> = vec![];
    let mut current = root.clone();
    while current.is_some() {
        res_vector.push(current.as_ref().unwrap().as_ref().borrow().val.clone());
        current = None;
    }
    res_vector
}
