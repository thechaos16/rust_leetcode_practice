use std::collections::HashMap;


fn main() {
    let word = "aazz".to_string();
    println!("{}", equal_frequency(word));
}

fn equal_frequency(word: String) -> bool {
    let mut hashmap = HashMap::new();
    for ch in word.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    if hashmap.len() == 1 {
        return true;
    }
    let mut count_map = HashMap::new();
    for (_, value) in &hashmap {
        *count_map.entry(value).or_insert(0) += 1;
    }
    if count_map.len() > 2 {
        return false;
    }
    if count_map.len() == 1 {
        if count_map.contains_key(&1) {
            return true;
        } else {
            return false;
        }
    }
    let mut value_1 = -1;
    for (key, value) in &count_map {
        if *value == 1 && value_1 < **key {
            if **key == 1 {
                return true;
            }
            if value_1 < **key {   
                value_1 = **key;
            }            
        }
    }
    if value_1 == -1 {
        return false;
    }
    if count_map.contains_key(&(value_1 - 1)) || value_1 -1 == 0 {
        return true;
    }
    return false;
}