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

fn main () {
    let mut root = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(2);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{}", is_symmetric(Some(Rc::new(RefCell::new(root)))));
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let mut left_vec = vec![];
    let mut right_vec = vec![];
    left_vec.push(root.clone().unwrap().borrow().left.clone());
    right_vec.push(root.clone().unwrap().borrow().right.clone());
    while left_vec.len() == right_vec.len() && left_vec.len() != 0 {
        let mut left_next = vec![];
        let mut right_next = vec![];
        for idx in 0..left_vec.len() {
            let left_node = left_vec[idx].clone();
            let right_node = right_vec[idx].clone();
            if (left_node.is_none() && !right_node.is_none()) || (!left_node.is_none() && right_node.is_none()) {
                return false;
            }
            if left_node.is_none() && right_node.is_none() {
                continue;
            }
            if left_node.clone().unwrap().borrow().val != right_node.clone().unwrap().borrow().val {
                return false;
            }
            left_next.push(left_node.clone().unwrap().borrow().left.clone());
            left_next.push(left_node.clone().unwrap().borrow().right.clone());
            
            right_next.push(right_node.clone().unwrap().borrow().right.clone());
            right_next.push(right_node.clone().unwrap().borrow().left.clone());
        }
        left_vec = left_next;
        right_vec = right_next;
    }
    return true;
}