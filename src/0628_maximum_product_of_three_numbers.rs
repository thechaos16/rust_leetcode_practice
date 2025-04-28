fn main() {
    let nums = vec![1, 2, 3];
    println!("{}", maximum_products(nums));
}

fn maximum_products(nums: Vec<i32>) -> i32 {
    if nums.len() == 3 {
        return nums[0] * nums[1] * nums[2];
    }
    let mut sorted = nums;
    sorted.sort();

    let n = sorted.len();
    let product1 = sorted[n - 1] * sorted[n - 2] * sorted[n - 3];
    let product2 = sorted[0] * sorted[1] * sorted[n - 1];
    product1.max(product2)
}