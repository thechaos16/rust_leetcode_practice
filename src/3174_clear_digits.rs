use std::iter::FromIterator;


fn main() {
    let s = "cb2a1a".to_string();
    println!("{}", clear_digits(s));
}

fn clear_digits(s: String) -> String {
    let mut ans = vec![];
    for ch in s.chars() {
        if ch >= '0' && ch <= '9' {
            _ = ans.pop();
        } else {
            ans.push(ch);
        }
    }
    return String::from_iter(ans);
}