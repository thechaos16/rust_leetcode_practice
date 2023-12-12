fn main() {
    let input = vec![3, 4, 5, 2];
    println!("{}", max_product(input));
}

fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_val = 0;
    let mut second_max = 0;
    for idx in 0..nums.len() {
        if nums[idx] > max_val {
            second_max = max_val;
            max_val = nums[idx];
        } else if nums[idx] > second_max {
            second_max = nums[idx];
        }
    }
    return (max_val - 1) * (second_max - 1);
}