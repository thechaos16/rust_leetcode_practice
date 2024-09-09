use std::cmp;

fn main() {
    let num1 = 1;
    let num2 = 10;
    let num3 = 1000;
    println!("{}", generate_key(num1, num2, num3));
}

fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let mut num1_mut = num1;
    let mut num2_mut = num2;
    let mut num3_mut = num3;
    let mut ans = 0;
    for idx in 0..4 {
        let min_val = cmp::min(cmp::min(num1_mut % 10, num2_mut % 10), num3_mut % 10);
        ans += min_val * 10_i32.pow(idx);
        num1_mut /= 10;
        num2_mut /= 10;
        num3_mut /= 10;
    }
    return ans;
}