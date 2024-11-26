fn main() {
    let n = 10;
    let m = 3;
    println!("{}", difference_of_sum(n, m));
}

fn difference_of_sum(n: i32, m: i32) -> i32 {
    let mut divisible = 0;
    let mut not_divisible = 0;
    for num in 1..(n + 1) {
        if num % m == 0 {
            divisible += num;
        } else { 
            not_divisible += num;
        }
    }
    return not_divisible - divisible;
}