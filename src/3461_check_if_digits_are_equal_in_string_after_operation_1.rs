fn main() {
    let s = "3902".to_string();
    println!("{}", has_same_digits(s));
}

fn has_same_digits(s: String) -> bool {
    if s.len() == 2 {
        return s.chars().nth(0) == s.chars().nth(1);
    }
    let mut ans = vec![];
    for idx in 0..s.len() - 1 {
        let plus = (s.chars().nth(idx).unwrap().to_digit(10).unwrap() + s.chars().nth(idx + 1).unwrap().to_digit(10).unwrap()) % 10;
        ans.push(plus.to_string());
    }
    return has_same_digits(ans.join(""));
}