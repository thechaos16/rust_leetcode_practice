use std::collections::HashMap;


fn main() {
    let input = "leetcode".to_string();
    println!("{}", first_uniq_char(input));
}

fn first_uniq_char(s: String) -> i32 {
    let mut hashmap = HashMap::new();
    for ch in s.chars() {
        let before_cnt = hashmap.entry(ch).or_insert(0);
        *before_cnt += 1;
    }
    for (idx, ch) in s.chars().enumerate() {
        if *hashmap.get(&ch).unwrap() == 1 {
            return idx as i32;
        }
    }
    return -1;
}