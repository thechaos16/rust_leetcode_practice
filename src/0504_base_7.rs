fn main () {
    let input = 0;
    println!("{}", convert_to_base7(input));
}

fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return '0'.to_string();
    }
    let mut string = String::new();
    let mut num2 = num.abs();
    while num2 > 0 {
        let x = (num2 % 7).to_string();
        string.push(x.chars().nth(0).unwrap());
        num2 /= 7;
    }
    if num < 0 {
        string.push('-');
    }
    return string.chars().rev().collect::<String>();
}