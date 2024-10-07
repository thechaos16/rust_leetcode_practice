fn main() {
    let s = "ABFCACDB".to_string();
    println!("{}", min_length(s));
}

fn min_length(s: String) -> i32 {
    let mut char_vec = vec![];
    for ch in s.chars() {
        if char_vec.len() == 0 {
            char_vec.push(ch);
        } else {
            let cur_char = char_vec.pop().unwrap();
            if (cur_char == 'A' && ch == 'B') || (cur_char == 'C' && ch == 'D') {
                continue;
            }
            char_vec.push(cur_char);
            char_vec.push(ch);
        }
    }
    return char_vec.len() as i32;
}