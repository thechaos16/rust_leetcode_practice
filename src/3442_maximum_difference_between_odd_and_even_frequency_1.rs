use std::collections::HashMap;


fn main() {
    let s= "aaaaabbc".to_string();
    println!("{}", max_difference(s));
}

fn max_difference(s: String) -> i32 {
    let mut hashmap = HashMap::new();
    for ch in s.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    let mut odd_max = 0;
    let mut even_min = s.len() as i32;
    for (key, val) in hashmap.into_iter() {
        if val % 2 == 0 {
            even_min = even_min.min(val);
        } else {
            odd_max = odd_max.max(val);
        }
    }
    return odd_max - even_min;
}