fn main() {
    let nums = vec![10, 10, 3, 7, 6];
    println!("{}", count_partitions(nums));
}

fn count_partitions(nums: Vec<i32>) -> i32 {
    let (mut left, mut cnt) = (0, 0);
    let mut all_sum: i32 = nums.iter().sum();
    for idx in 0..(nums.len() - 1) {
        left += nums[idx];
        all_sum -= nums[idx];
        if (left - all_sum).abs() % 2 == 0 {
            cnt += 1;
        }
    }
    return cnt;
}