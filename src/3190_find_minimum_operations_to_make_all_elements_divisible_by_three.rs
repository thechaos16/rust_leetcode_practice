fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("{}", minimum_opertions(nums));
}

fn minimum_opertions(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    for num in nums {
        if num % 3 == 0 {
            cnt += 0;
        } else {
            cnt += 1;
        }
    }
    return cnt;
}