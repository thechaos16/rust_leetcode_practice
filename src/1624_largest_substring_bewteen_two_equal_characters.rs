use std::collections::HashMap;


fn main() {
    let input = "cbzxy".to_string();
    println!("{}", max_length_between_equal_charcters(input));
}

fn max_length_between_equal_charcters(s: String) -> i32 {
    let mut hashmap: HashMap<char, i32> = HashMap::with_capacity(26);
    let mut max_val = -1;
    for (idx, ch) in s.chars().enumerate() {
        match hashmap.get(&ch) {
            Some(&value) => {
                let cur_val = idx as i32 - value - 1;
                max_val = max_val.max(cur_val);
            }
            None => {
                hashmap.insert(ch, idx as i32);
            }
        }
    }
    max_val
}