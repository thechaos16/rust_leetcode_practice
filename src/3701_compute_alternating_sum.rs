fn main() {
    let nums = vec![1, 3, 5, 7];
    println!("{}", alternating_sum(nums));
}

fn alternating_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for idx in 0..nums.len() {
        if idx % 2 == 0 {
            ans += nums[idx];
        } else {
            ans -= nums[idx];
        }
    }
    return ans;
}