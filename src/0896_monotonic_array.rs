fn main() {
    let input = vec![1, 2, 2, 3];
    println!("{}", is_monotonic(input));
}

fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut last = 0;
    for idx in 1..nums.len() {
        let cur = nums[idx] - nums[idx - 1];
        if cur == 0 {
            continue;
        } else if cur > 0 {
            if last == -1 {
                return false;
            } else {
                last = 1;
            }
        } else {
            if last == 1 {
                return false;
            } else {
                last = -1;
            }
        }
    }
    return true;
}