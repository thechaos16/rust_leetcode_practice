use std::collections::HashMap;


fn main() {
    let string1 = "egg".to_string();
    let string2 = "add".to_string();
    println!("{}", is_isomorphic(string1, string2));
}

fn is_isomorphic(s: String, t: String) -> bool {
    let mut hash1 = HashMap::new();
    let mut hash2 = HashMap::new();
    let mut li1 = vec![];
    let mut li2 = vec![];
    let mut idx: i32 = 0;
    for ch in s.chars() {
        if !hash1.contains_key(&ch) {
            hash1.insert(ch, idx);
            idx += 1;
        }
        let cur_val: i32 = *hash1.get(&ch).unwrap();
        li1.push(cur_val);
    }
    idx = 0;
    for ch in t.chars() {
        if !hash2.contains_key(&ch) {
            hash2.insert(ch, idx);
            idx += 1;
        }
        let cur_val: i32 = *hash2.get(&ch).unwrap();
        li2.push(cur_val);
    }
    for idx in 0..li1.len() {
        if li1[idx] != li2[idx] {
            return false;
        }
    }
    return true;
}