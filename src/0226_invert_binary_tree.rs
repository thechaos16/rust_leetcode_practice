use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(val: i32) -> Self { 
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{}", invert_tree(Some(Rc::new(RefCell::new(root)))).clone().unwrap().borrow().val);
}

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut new_root = TreeNode::new(root.clone().unwrap().borrow().val);
    let new_right = invert_tree(root.clone().unwrap().borrow().left.clone());
    let new_left = invert_tree(root.clone().unwrap().borrow().right.clone());
    new_root.left = new_left;
    new_root.right = new_right;
    return Some(Rc::new(RefCell::new(new_root)));
}