use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let input = "aaabc".to_string();
    println!("{}", minimized_string_length(input));
}

fn minimized_string_length(s: String) -> i32 {
    let hash: HashSet<char> = HashSet::from_iter(s.chars());
    return hash.len() as i32;
}