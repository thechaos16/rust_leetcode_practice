use std::collections::HashSet;

fn main() {
    let input = vec!["aba".to_string(), "aabb".to_string(), "abcd".to_string(), "bac".to_string(), "aabc".to_string()];
    println!("{}", similar_pairs(input));
}

fn similar_pairs(words: Vec<String>) -> i32 {
    let mut set_vec = vec![];
    for word in words {
        let set1: HashSet<char> = word.chars().collect();
        set_vec.push(set1);
    }
    let mut cnt = 0;
    for idx in 0..set_vec.len() {
        for idx2 in (idx + 1)..set_vec.len() {
            if set_vec[idx] == set_vec[idx2] {
                cnt += 1;
            }
        }
    }
    return cnt;
}