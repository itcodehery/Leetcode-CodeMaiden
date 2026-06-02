use std::cell::RefCell;
use std::rc::Rc;

pub struct ConfigNode {
    pub key: String,
    pub is_locked: bool,
    pub left: Option<Rc<RefCell<ConfigNode>>>,
    pub right: Option<Rc<RefCell<ConfigNode>>>,
}

impl ConfigNode {
    // Helper to create a new node
    pub fn new(key: &str) -> Option<Rc<RefCell<ConfigNode>>> {
        Some(Rc::new(RefCell::new(ConfigNode {
            key: key.to_string(),
            is_locked: false,
            left: None,
            right: None,
        })))
    }
}

// TODO: Traverse the tree to find the `target_key`.
// If found: Mutate `is_locked` to true, and return a clone of the Rc pointer.
// If not found: Return None.
pub fn lock_and_retrieve(
    root: Option<Rc<RefCell<ConfigNode>>>,
    target_key: &str,
) -> Option<Rc<RefCell<ConfigNode>>> {
    // 1. Presence: Peel the option. If it's None, we hit the bottom of a branch, return None.
    let requested_node = root?;

    // 2. Permission (Read): Borrow the node to check if `key == target_key`.
    // 3. Permission (Write): If it matches, borrow mutably, change `is_locked = true`.
    //    Ownership: Return `Some(node.clone())` so the caller gets shared ownership.
    if requested_node.borrow().key == target_key {
        requested_node.borrow_mut().is_locked = true;
        return Some(requested_node.clone());
    }

    // 4. Ownership (Recursion): If it didn't match, we need to search the left and right children.
    //    We need to pass the children into `lock_and_retrieve`. Because we are passing them
    //    into a function, we must `.clone()` the child pointers!
    let left_child = requested_node.borrow().left.clone();
    let right_child = requested_node.borrow().right.clone();

    if let Some(left) = lock_and_retrieve(left_child, target_key) {
        return Some(left);
    }

    if let Some(right) = lock_and_retrieve(right_child, target_key) {
        return Some(right);
    }

    None
}

fn main() {
    // Building a tiny tree:
    //       "theme"
    //       /     \
    // "keybinds" "plugins"

    let root = ConfigNode::new("theme");
    let left = ConfigNode::new("keybinds");
    let right = ConfigNode::new("plugins");

    // Linking them up
    if let Some(r) = &root {
        r.borrow_mut().left = left;
        r.borrow_mut().right = right;
    }

    // Test the function
    let found_node = lock_and_retrieve(root.clone(), "plugins");

    if let Some(node) = found_node {
        println!("Found and locked: {}", node.borrow().key);
        assert_eq!(node.borrow().is_locked, true);
    } else {
        println!("Key not found.");
    }
}
