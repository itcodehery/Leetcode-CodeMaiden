use text_io::read;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

fn add_node(head: &mut TreeNode) {
    let ch: i32;
    let input: i32;
    let mut current = head;
    if current.left.is_none() && current.right.is_none() {
        println!("Which node? (0/1): ");
        ch = read!();
        if ch == 0 {
            println!("Enter a value: ");
            input = read!();
            current.left = Some(Box::new(TreeNode::new(input)));
            current = current.left.as_mut().unwrap();
        } else {
            println!("Enter a value: ");
            input = read!();
            current.right = Some(Box::new(TreeNode::new(input)));
            current = current.right.as_mut().unwrap();
        }
    } else if current.left.is_none() {
        println!("Enter a value for the left node: ");
        input = read!();
        current.left = Some(Box::new(TreeNode::new(input)));
        current = current.left.as_mut().unwrap();
    } else if current.right.is_none() {
        println!("Enter a value for the right node: ");
        input = read!();
        current.right = Some(Box::new(TreeNode::new(input)));
        current = current.right.as_mut().unwrap();
    }
}

fn inorder_traversal(head: &mut TreeNode) {
    inorder_traversal(head.left.as_mut().unwrap());
    println!("{}", head.val);
    inorder_traversal(head.right.as_mut().unwrap());
}

fn main() {
    let mut head = TreeNode::new(0);
    add_node(&mut head);
    inorder_traversal(&mut head);
}
