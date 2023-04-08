
fn main() {
    let input: i32 = 101;
    let input2: i32 = -101;
    println!("{}", is_palindrome(input));
    println!("{}", is_palindrome(input2));
}

fn is_palindrome(input: i32) -> bool {
    let input_str = input.to_string();
    let reverse_str = input_str.chars().rev().collect::<String>();
    let mut result: bool = true;
    for idx in 0..input_str.len() {
        if input_str.chars().nth(idx).unwrap() != reverse_str.chars().nth(idx).unwrap() {
            result = false;
            break
        }
    }
    return result;
}