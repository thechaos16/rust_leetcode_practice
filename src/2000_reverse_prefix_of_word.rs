fn main() {
    let input = "abcdefd".to_string();
    let ch = "d".to_string();
    println!("{}", reverse_prefix(input, ch.chars().nth(0).unwrap()));
}

fn reverse_prefix(word: String, ch: char) -> String {
    let mut split_idx = -1;
    for idx in 0..word.len() {
        if word.chars().nth(idx).unwrap() == ch {
            split_idx = idx as i32;
            break;
        }
    }
    if split_idx == -1 {
        return word;
    }
    let mut new_str = String::new();
    for idx in (0..(split_idx as usize + 1)).rev() {
        new_str.push(word.chars().nth(idx).unwrap());
    }
    for idx in (split_idx as usize + 1)..word.len() {
        new_str.push(word.chars().nth(idx).unwrap());
    }
    return new_str;
}