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
        TreeNode{
            val,
            left: None,
            right: None
        }
    }
}

fn main () {
    let mut root = TreeNode::new(10);
    let mut left = TreeNode::new(5);
    let left2 = TreeNode::new(3);
    left.left = Some(Rc::new(RefCell::new(left2)));
    root.left = Some(Rc::new(RefCell::new(left)));
    println!("{:?}", inorder_traversal(Some(Rc::new(RefCell::new(root)))));
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    if root.is_none() {
        return vec![];
    }
    let left_res = &mut inorder_traversal(root.clone().unwrap().borrow().left.clone());
    let right_res = &mut inorder_traversal(root.clone().unwrap().borrow().right.clone());
    res.append(left_res);
    res.push(root.clone().unwrap().borrow().val);
    res.append(right_res);
    return res
}