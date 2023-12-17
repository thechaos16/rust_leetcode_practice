fn main() {
    let input = vec![1, 2, 2, 3];
    println!("{}", is_monotonic(input));
}

fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut last = 0;
    for window in nums.windows(2) {
        let cur = (window[1] - window[0]).signum();
        if last != 0 && cur != 0 && cur != last {
            return false;
        }
        if cur != 0 {
            last = cur;
        }
    }
    true
}