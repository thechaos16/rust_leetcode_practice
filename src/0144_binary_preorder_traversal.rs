// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
    let mut right = TreeNode::new(2);
    let left2 = TreeNode::new(3);
    right.left = Some(Rc::new(RefCell::new(left2)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{:?}", preorder_traversal(Some(Rc::new(RefCell::new(root)))));
}

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    if root.is_none() {
        return ans;
    }
    ans.append(&mut vec![root.clone().unwrap().borrow().val]);
    ans.append(&mut preorder_traversal(root.clone().unwrap().borrow().left.clone()));
    ans.append(&mut preorder_traversal(root.clone().unwrap().borrow().right.clone()));
    return ans;
}