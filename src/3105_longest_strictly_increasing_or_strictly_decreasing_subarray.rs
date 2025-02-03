use std::cmp;


fn main() {
    let nums = vec![1, 4, 3, 3, 2];
    println!("{}", longest_monotonic_subarray(nums));
}

fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let (mut ans, mut cur_len, mut order) = (1, 0, 0);
    for idx in 0..(nums.len() - 1) {
        if nums[idx] == nums[idx + 1] {
            ans = cmp::max(ans, cur_len);
            order = 0;
            cur_len = 0;
        } else if nums[idx] > nums[idx + 1] {
            if order == -1 {
                cur_len += 1;
            } else {
                ans = cmp::max(ans, cur_len);
                order = -1;
                cur_len = 2;
            }
        } else {
            if order == 1 {
                cur_len += 1;
            } else {
                ans = cmp::max(ans, cur_len);
                order = 1;
                cur_len = 2;
            }
        }
    }
    return cmp::max(ans, cur_len);
}