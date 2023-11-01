fn main() {
    let input = 121;
    println!("{}", count_digits(input));
}

fn count_digits(num: i32) -> i32 {
    let mut number = num;
    let mut count = 0;
    while number != 0 {
        let cur_num = number % 10;
        if num % cur_num == 0 {
            count += 1;
        }
        number /= 10;
    }
    return count;
}