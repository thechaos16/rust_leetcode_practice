use std::collections::HashMap;


fn main() {
    let string1 = "egg".to_string();
    let string2 = "add".to_string();
    println!("{}", is_isomorphic(string1, string2));
}

fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut hash1 = HashMap::with_capacity(s.len());
    let mut hash2 = HashMap::with_capacity(t.len());

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        match (hash1.get(&s_char), hash2.get(&t_char)) {
            (None, None) => {
                hash1.insert(s_char, t_char);
                hash2.insert(t_char, s_char);
            },
            (Some(&mapped_t), Some(&mapped_s)) => {
                if mapped_t != t_char || mapped_s != s_char {
                    return false;
                }
            },
            _ => return false,
        }
    }
    true
}