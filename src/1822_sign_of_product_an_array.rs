fn main() {
    let input = vec![-1,-2,-3,-4,3,2,1];
    println!("{}", array_sign(input));
}

fn array_sign(nums: Vec<i32>) -> i32 {
    let mut minus_cnt = 0;
    for idx in 0..nums.len() {
        if nums[idx] == 0 {
            return 0;
        } else if nums[idx] < 0 {
            minus_cnt += 1;
        }
    }
    if minus_cnt % 2 == 0 {
        return 1;
    } else {
        return -1;
    }
}