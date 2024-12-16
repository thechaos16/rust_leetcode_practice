use std::collections::BinaryHeap;


fn main() {
    let nums = vec![10, 10, 10, 10, 10];
    let k = 5;
    println!("{}", max_kelements(nums, k));
}

fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap = BinaryHeap::from(nums);
    let mut ans = 0 as i64;
    for _ in 0..k {
        let cur = heap.pop().unwrap();
        ans += cur as i64;
        heap.push((cur as f64 / 3.0).ceil() as i32);
    }
    return ans;
}