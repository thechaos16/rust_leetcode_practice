use std::collections::HashMap;


fn main () {
    let input1 = "anagqam".to_string();
    let input2 = "nagrram".to_string();
    println!("{}", is_anagram(input1, input2));
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut cnt1 = HashMap::new();
    let mut cnt2 = HashMap::new();
    for ch in s.chars() {
        let number = cnt1.entry(ch).or_insert(0);
        *number += 1;
    }
    for ch in t.chars() {
        let number = cnt2.entry(ch).or_insert(0);
        *number += 1;
    }
    
    let counter_vec1: Vec<(&char, &i32)> = cnt1.iter().collect();
    for (a, b) in counter_vec1 {
        match cnt2.get(&a) {
           Some(child) => {
               if *b != cnt2[a] {
                   return false;
               }
           },
           None => {
               return false;
           }
        }
    }
    return true;
}
