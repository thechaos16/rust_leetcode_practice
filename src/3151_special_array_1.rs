fn main() {
    let nums = vec![1];
    println!("{}", is_array_special(nums));
}

fn is_array_special(nums: Vec<i32>) -> bool {
    for idx in 0..(nums.len() - 1) {
        if (nums[idx] % 2 + nums[idx + 1] % 2) % 2 == 0 {
            return false;
        }
    }
    return true;
}