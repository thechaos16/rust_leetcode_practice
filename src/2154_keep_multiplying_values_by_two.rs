use std::collections::HashSet;

fn main() {
    let nums = vec![5, 3, 6, 1, 12];
    let original= 3;
    println!("{}", find_final_value(nums, original));
}

fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let hash: HashSet<i32> = nums.into_iter().collect();
    let mut orig_mut = original;
    while hash.contains(&orig_mut) {
        orig_mut *= 2;
    }
    return orig_mut;
}