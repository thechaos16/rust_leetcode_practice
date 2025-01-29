fn main() {
    let nums = vec![2, 1, 4, 3];
    println!("{}", semi_ordered_permutation(nums));
}

fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let (mut one_idx, mut n_idx) = (0, 0);
    for idx in 0..nums.len() {
        if nums[idx] == 1 {
            one_idx = idx as i32;
        } else if nums[idx] == nums.len() as i32 {
            n_idx = idx as i32;
        }
    }
    return one_idx + (nums.len() as i32 - 1 - n_idx) - (one_idx > n_idx) as i32;
}