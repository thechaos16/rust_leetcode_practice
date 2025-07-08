fn main() {
    let nums = vec![1];
    let k = 0;
    println!("{}", smallest_range_i(nums, k));
}

fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let (mut min_val, mut max_val) = (10000, 0);
    for num in nums {
        min_val = min_val.min(num);
        max_val = max_val.max(num);
    }
    return (max_val - min_val - 2 * k).max(0);
}