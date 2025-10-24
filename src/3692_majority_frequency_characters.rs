use std::collections::HashMap;


fn main() {
    let s = "aaabbbccdddde".to_string();
    println!("{}", majority_frequency_group(s));
}

fn majority_frequency_group(s: String) -> String {
    let mut hashmap = HashMap::new();
    for ch in s.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    let mut count_count = HashMap::new();
    let (mut max_val, mut max_count) = (0, 0);
    for (_, val) in hashmap.clone().into_iter() {
        *count_count.entry(val).or_insert(0) += 1;
        let cur_val = *count_count.entry(val).or_insert(0);
        if cur_val > max_count {
            max_count = cur_val;
            max_val = val;
        } else if cur_val == max_count {
            if val > max_val {
                max_val = val;
            }
        }
    }
    let mut vector = vec![];
    for (key, val) in hashmap.into_iter() {
        if val == max_val {
            vector.push(key.to_string());
        }
    }
    return vector.join("");
}