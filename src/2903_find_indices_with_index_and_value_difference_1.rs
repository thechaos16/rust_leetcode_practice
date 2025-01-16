fn main() {
    let nums = vec![5, 1, 4, 1];
    let index_difference = 2;
    let value_difference = 4;
    println!("{:?}", find_indices(nums, index_difference, value_difference));
}

fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let mut ans = vec![-1, -1];
    for idx in 0..nums.len() {
        for idx2 in idx..nums.len() {
            if (idx2 as i32 - idx as i32) >= index_difference && (nums[idx] - nums[idx2]).abs() >= value_difference {
                ans[0] = idx as i32;
                ans[1] = idx2 as i32;
                return ans;
            }
        }
    }
    return ans;
}