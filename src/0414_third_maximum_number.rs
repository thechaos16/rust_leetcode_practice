use std::collections::BTreeSet;

fn main() {
    let nums = vec![3, 2, 1];
    println!("{}", third_max(nums));
}

fn third_max(nums: Vec<i32>) -> i32 {
    let mut unique_nums: BTreeSet<i32> = nums.iter().copied().collect();
    if unique_nums.len() < 3 {
        return *unique_nums.last().unwrap();
    }
    _ = unique_nums.pop_last();
    _ = unique_nums.pop_last();
    *unique_nums.last().unwrap()
}
