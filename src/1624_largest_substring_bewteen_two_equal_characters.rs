use std::cmp;
use std::collections::HashMap;


fn main() {
    let input = "cbzxy".to_string();
    println!("{}", max_length_between_equal_charcters(input));
}

fn max_length_between_equal_charcters(s: String) -> i32 {
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    let mut max_val = -1;
    for (idx, ch) in s.chars().enumerate() {
        if hashmap.contains_key(&ch) {
            let cur_val = (idx as i32) - hashmap.get(&ch).unwrap() - 1;
            max_val = cmp::max(max_val, cur_val);
        } else {
            hashmap.insert(ch, idx as i32);
        }
    }
    return max_val;
}