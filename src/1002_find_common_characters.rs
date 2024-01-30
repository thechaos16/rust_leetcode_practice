use std::collections::HashMap;


fn main() {
    let words = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
    println!("{:?}", common_chars(words));
}

fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut ans = vec![];
    let mut cur_hash = get_hashmap(words[0].clone());
    for idx in 1..words.len() {
        let hash = get_hashmap(words[idx].clone());
        for (key, value) in cur_hash.clone().iter() {
            if !hash.contains_key(key) {
                cur_hash.remove(key);
            } else if *hash.get(key).unwrap() < *value {
                cur_hash.insert(*key, *hash.get(key).unwrap());
            }
        }
    }
    for (key, value) in cur_hash.clone().iter() {
        for _idx in 0..*value {
            ans.push(key.to_string());
        }
    }
    return ans;
}

fn get_hashmap(word: String) -> HashMap<char, i32> {
    let mut hashmap = HashMap::new();
    for ch in word.chars() {
        let before_count = hashmap.entry(ch).or_insert(0);
        *before_count += 1;
    }
    return hashmap;
}