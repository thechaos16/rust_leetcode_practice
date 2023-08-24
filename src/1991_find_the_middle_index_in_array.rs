fn main() {
    let input = vec![2,5];
    println!("{}", find_middle_index(input));
}

fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut sum_ = 0;
    let mut cur_sum = vec![];
    for num in &nums {
        sum_ += num;
        cur_sum.push(sum_);
    }
    for idx in 0..nums.len() {
        if idx == 0 || idx == nums.len() - 1 {
            if sum_ - nums[idx] == 0 {
                return idx as i32;
            }
        } else {
            let left = cur_sum[idx - 1];
            let right = cur_sum[nums.len() - 1] - cur_sum[idx];
            if left == right {
                return idx as i32;
            }
        }
    }
    return -1;
}