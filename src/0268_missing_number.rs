fn main() {
    let input = vec![3, 0, 1];
    let input2 = vec![9,6,4,2,3,5,7,0,1];
    println!("{}", find_number(input));
    println!("{}", find_number(input2));
}

fn find_number(nums: Vec<i32>) -> i32 {
    let target = nums.len() * (nums.len() + 1) / 2;
    let mut sum = 0;
    for idx in 0..nums.len() {
        sum += nums[idx];
    }
    return target as i32 - sum;
}