use std::collections::HashMap;


fn main () {
    let words = vec!["cat".to_string(), "bt".to_string(), "hat".to_string(), "tree".to_string()];
    let chars = "atach".to_string();
    println!("{}", count_characters(words, chars));
}

fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut hashmap = HashMap::new();
    for ch in chars.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    let mut cnt = 0;
    for word in words {
        let mut word_hashmap = HashMap::new();
        let mut correct = true;
        for ch in word.chars() {
            *word_hashmap.entry(ch).or_insert(0) += 1;
        }
        for (key, value) in word_hashmap {
            if value > *hashmap.entry(key).or_insert(0) {
                correct = false;
                break;
            }
        }
        if correct {
            cnt += word.len();
        }
    }
    return cnt as i32;
}