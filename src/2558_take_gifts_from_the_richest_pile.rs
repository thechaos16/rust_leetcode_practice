use std::collections::BinaryHeap;


fn main() {
    let gifts = vec![25,64,9,4,100];
    let k = 4;
    println!("{}", pick_gifts(gifts, k));
}

fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap = BinaryHeap::<i32>::new();
    for gift in gifts.iter() {
        heap.push(*gift);
    }
    for idx in 0..k {
        let cur = heap.pop().unwrap();
        let after = (cur as f64).sqrt() as i32;
        heap.push(after);
    }
    let mut ans = 0;
    for idx in 0..heap.len() {
        ans += heap.pop().unwrap() as i64;
    }
    return ans;
}