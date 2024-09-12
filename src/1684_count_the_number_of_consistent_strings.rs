use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let allowed = "ab".to_string();
    let words = vec!["ad".to_string(), "bd".to_string(), "aaab".to_string(), "baa".to_string(), "badab".to_string()];
    println!("{}", count_consistent_strings(allowed, words));
}

fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut cnt = 0;
    let allowed_set: HashSet<char> = HashSet::from_iter(allowed.chars());
    for word in words.iter() {
        for ch in word.chars() {
            if !allowed_set.contains(&ch) {
                cnt += 1;
                break;
            }
        }
    }
    return words.len() as i32 - cnt;
}