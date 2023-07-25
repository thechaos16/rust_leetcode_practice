use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
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
    let left = TreeNode::new(5);
    root.left = Some(Rc::new(RefCell::new(left)));
    println!("{}", is_unival_tree(Some(Rc::new(RefCell::new(root)))));
}

fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let left = root.clone().unwrap().borrow().left.clone();
    let right = root.clone().unwrap().borrow().right.clone();
    if left.is_none() {
        if right.clone().is_none() {
            return true;
        } else {
            return is_unival_tree(right.clone()) && root.clone().unwrap().borrow().val == right.clone().unwrap().borrow().val;
        }
    } else {
        if right.is_none() {
            return is_unival_tree(left.clone()) && root.clone().unwrap().borrow().val == left.clone().unwrap().borrow().val;
        } else {
            return is_unival_tree(left.clone()) && root.clone().unwrap().borrow().val == left.clone().unwrap().borrow().val && is_unival_tree(right.clone()) && root.clone().unwrap().borrow().val == right.clone().unwrap().borrow().val;
        }
    }
}