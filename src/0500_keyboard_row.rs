use std::collections::HashMap;


fn main() {
    let input = vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()];
    println!("{:?}", find_words(input));
}


fn find_words(words: Vec<String>) -> Vec<String> {
    let rows = vec!["qwertyuiop".to_string(), "asdfghjkl".to_string(), "zxcvbnm".to_string()];
    let mut hashmap = HashMap::new();
    for idx in 0..rows.len() {
        for ch in rows[idx].chars() {
            hashmap.insert(ch, idx);
        }
    }
    let mut ans = vec![];
    for word in words {
        let mut cur_idx = 5 as usize;
        let mut good = true;
        for ch in word.to_lowercase().chars() {
            let new_idx = *hashmap.get(&ch).unwrap();
            if cur_idx == 5 as usize {
                cur_idx = new_idx;
            } else if cur_idx != new_idx {
                good = false;
                break
            }
        }
        if good {
            ans.push(word);
        }
    }
    return ans
}