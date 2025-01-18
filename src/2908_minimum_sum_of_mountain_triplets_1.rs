fn main() {
    let nums = vec![8, 6, 1, 5, 3];
    println!("{}", minimum_sum(nums));
}

fn minimum_sum(nums: Vec<i32>) -> i32 {
    let (mut left_min, mut right_min) = (vec![0; nums.len()], vec![0; nums.len()]);
    let (mut cur_left_min, mut cur_right_min) = (51, 51);
    for idx in 0..nums.len() {
        left_min[idx] = cur_left_min;
        if cur_left_min > nums[idx] {
            cur_left_min = nums[idx];
        }
        right_min[nums.len() - idx - 1] = cur_right_min;
        if cur_right_min > nums[nums.len() - idx - 1] {
            cur_right_min = nums[nums.len() - idx - 1];
        }
    }
    let mut ans = 151;
    for idx in 0..nums.len() {
        if left_min[idx] < nums[idx] && right_min[idx] < nums[idx] {
            if ans > nums[idx] + left_min[idx] + right_min[idx] {
                ans = nums[idx] + left_min[idx] + right_min[idx];
            }
        }
    }
    if ans == 151 {
        return -1;
    } else {
        return ans;
    }
}