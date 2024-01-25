use std::iter::FromIterator;


fn main() {
    let input = "abbaca".to_string();
    println!("{:?}", remove_duplicates(input));
}

fn remove_duplicates(s: String) -> String {
    let mut ch_vec = vec![];
    for ch in s.chars() {
        if ch_vec.len() >= 1 && ch == ch_vec[ch_vec.len() - 1] {
            _ = ch_vec.pop();
        } else {
            ch_vec.push(ch);
        }
    }
    return String::from_iter(&ch_vec);
}