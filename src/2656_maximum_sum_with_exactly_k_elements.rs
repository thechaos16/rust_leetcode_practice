fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let k = 3;
    println!("{}", maximize_sum(nums, k));
}

fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let max_val = nums.iter().max().unwrap();
    let cnt = max_val * k + (k * (k - 1)) / 2;
    return cnt;
}