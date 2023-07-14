fn main() {
    let input = vec![1, 7, 3, 6, 5, 6];
    println!("{}", pivot_index(input));
}

fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum_ = 0;
    for num in nums.clone().into_iter() {
        sum_ += num;
    }
    let mut cur_sum = 0;
    for idx in 0..nums.len() {
        let temp_val = sum_ - nums[idx];
        if temp_val % 2 == 0 && cur_sum == temp_val / 2 {
            return idx as i32;
        }
        cur_sum += nums[idx];
    }
    return -1;
}