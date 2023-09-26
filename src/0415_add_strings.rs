use std::ops::Add;

fn main() {
    let number1 = "11".to_string();
    let number2 = "123".to_string();
    println!("{}", add_strings(number1, number2));
}

fn add_strings(num1: String, num2: String) -> String {
    let mut plus = 0;
    let mut ans = "".to_string();
    let mut max_len = 0;
    if num1.len() >= num2.len() {
        max_len = num1.len();
    } else {
        max_len = num2.len();
    }
    for idx in 0..max_len {
        let mut cur = 0;
        if num1.len() >= idx + 1 {
            cur += num1.chars().nth(num1.len() - idx - 1).unwrap().to_digit(10).unwrap();
        }
        if num2.len() >= idx + 1 {
            cur += num2.chars().nth(num2.len() - idx - 1).unwrap().to_digit(10).unwrap();
        }
        cur += plus;
        plus = cur / 10;
        cur = cur % 10;
        ans = cur.to_string().add(&ans);
    }
    if plus == 1 {
        ans = plus.to_string().add(&ans);
    }
    return ans;
}