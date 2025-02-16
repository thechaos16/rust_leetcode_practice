fn main() {
    let nums = vec![1, 3, 2, 1, 5, 4];
    let k = 2;
    println!("{}", sum_of_good_numbers(nums, k));
}

fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let uk = k as usize;
    for idx in 0..nums.len() {
        if (idx < uk || nums[idx] > nums[idx - uk]) && (idx + uk >= nums.len() || nums[idx] > nums[idx + uk]) {
            ans += nums[idx];
        }
    }
    ans
}