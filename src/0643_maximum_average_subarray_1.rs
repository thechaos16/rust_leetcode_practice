fn main () {
    let input = vec![1,12,-5,-6,50,3];
    let k = 4;
    println!("{}", find_max_average(input, k));
}

fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum_ = 0;
    for idx in 0..(k as usize) {
        sum_ += nums[idx];
    }
    let mut max_sum = sum_;
    for idx in (k as usize)..nums.len() {
        sum_ += nums[idx];
        sum_ -= nums[idx - (k as usize)];
        if sum_ > max_sum {
            max_sum = sum_;
        }
    }
    return max_sum as f64 / k as f64;
}