use std::collections::HashMap;


fn main() {
    let str1 = "abcdef".to_string();
    let str2 = "abcdefs".to_string();
    println!("{}", find_the_difference(str1, str2));
}

fn find_the_difference(s: String, t: String) -> char {
    let mut s_count = HashMap::new();
    let mut t_count = HashMap::new();
    for ch in s.chars() {
        let temp = s_count.entry(ch).or_insert(0);
        *temp += 1;
    }
    for ch in t.chars() {
        let temp = t_count.entry(ch).or_insert(0);
        *temp += 1;
    }
    for t_key in t_count {
        let s_val = s_count.entry(t_key.0).or_insert(0);
        if *s_val == 0 || t_key.1 != *s_val {
            return t_key.0;
        }
    }
    return 'a';
}