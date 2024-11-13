fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("{}", has_trailing_zeros(nums));
}

fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    let mut cnt = 0;
    for num in nums.iter() {
        if num % 2 == 0 {
            cnt += 1;
            if cnt >= 2 {
                return true;
            }
        }
    }
    return false;
}