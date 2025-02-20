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
    match root {
        None => true,
        Some(node) => {
            let borrowed = node.borrow();
            is_symmetric_subtrees(&borrowed.left, &borrowed.right)
        }
    }
}

fn is_symmetric_subtrees(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            let l_borrowed = l.borrow();
            let r_borrowed = r.borrow();
            l_borrowed.val == r_borrowed.val && is_symmetric_subtrees(&l_borrowed.left, &r_borrowed.right) && is_symmetric_subtrees(&l_borrowed.right, &r_borrowed.left)
        },
        _ => false
    }
}