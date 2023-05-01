fn main() {
    let input = 38;
    println!("{}", add_digits(input));
}

fn add_digits(num: i32) -> i32 {
    if num < 10 {
        return num;
    }
    let mut sum = 0;
    let mut mut_num = num;
    while mut_num != 0 {
        sum += mut_num % 10;
        mut_num /= 10;
    }
    return add_digits(sum);
}