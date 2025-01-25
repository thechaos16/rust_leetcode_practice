fn main() {
    let nums = vec![1, 2, 4];
    println!("{}", max_adjacent_difference(nums));
}

fn max_adjacent_difference(nums: Vec<i32>) -> i32 {
    let mut diff = 0;
    for idx in 0..nums.len() {
        let cur_diff = (nums[idx] - nums[(idx + 1) % nums.len()]).abs();
        if cur_diff >= diff {
            diff = cur_diff;
        }
    }
    return diff;
}
