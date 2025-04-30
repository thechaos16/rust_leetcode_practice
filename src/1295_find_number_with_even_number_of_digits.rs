fn main() {
    let nums = vec![12, 345, 2, 6, 7896];
    println!("{}", find_numbers(nums));
}

fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    for num in nums.into_iter() {
        if (num as f32).log10().floor() as i32 % 2 != 0 {
            cnt += 1;
        }
    }
    cnt
}