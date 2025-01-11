use std::collections::HashMap;


fn main() {
    let s = "annabelle".to_string();
    let k = 2;
    println!("{}", can_construct(s, k));
}

fn can_construct(s: String, k: i32) -> bool {
    if (s.len() as i32) < k {
        return false;
    }
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    let mut cnt = 0;
    for ch in s.chars() {
        *hashmap.entry(ch).or_default() += 1;
    }
    for val in hashmap.values() {
        if val % 2 == 1 {
            cnt += 1;
        }
    }
    return cnt <= k;
}