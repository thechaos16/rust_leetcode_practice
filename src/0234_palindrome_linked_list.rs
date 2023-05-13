// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
//   #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
    pub fn vec_to_list(mut value: Vec<i32>) -> Option<Box<ListNode>> {
        match value.pop() {
            Some(x) => Some(Box::new(ListNode {
                val: x,
                next: ListNode::vec_to_list(value),
            })),
            None => None,
        }
    }
}

fn main() {
    let mut input = ListNode::vec_to_list(vec![1, 2, 2, 1]);
    println!("{}", is_palindrome(input));
    let mut input = ListNode::vec_to_list(vec![1, 4, 2]);
    println!("{}", is_palindrome(input));
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut result = Vec::new();
    let mut head2 = head;
    while let Some(n) = head2 {
        result.push(n.val);
        head2 = n.next;
    }
    let length = result.len();
    for idx in 0..length / 2 {
        if result[idx] != result[length - idx - 1] {
            return false;
        }
    }
    return true;
}