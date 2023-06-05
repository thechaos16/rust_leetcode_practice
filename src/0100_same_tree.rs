use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode{
            val,
            left: None,
            right:None
        }
    }
}

fn main () {
    let mut one_tree =TreeNode::new(1);
    let mut two_tree = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    let left2 = TreeNode::new(2);
    let right2 = TreeNode::new(5);
    one_tree.left = Some(Rc::new(RefCell::new(left)));
    one_tree.right = Some(Rc::new(RefCell::new(right)));
    two_tree.left = Some(Rc::new(RefCell::new(left2)));
    two_tree.right = Some(Rc::new(RefCell::new(right2)));
    
    println!("{}", is_same_tree(Some(Rc::new(RefCell::new(one_tree))), Some(Rc::new(RefCell::new(two_tree)))));
}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    } else if !p.is_none() && !q.is_none() {
        let left_same = &mut is_same_tree(p.clone().unwrap().borrow().left.clone(), q.clone().unwrap().borrow().left.clone());
        let right_same = &mut is_same_tree(p.clone().unwrap().borrow().right.clone(), q.clone().unwrap().borrow().right.clone());
        let val_same = p.clone().unwrap().borrow().val == q.clone().unwrap().borrow().val;
        return *left_same && *right_same && val_same;
    } else {
        return false;
    }
}