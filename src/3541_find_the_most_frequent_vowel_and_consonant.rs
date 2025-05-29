use std::collections::HashMap;


fn main() {
    let s = "successes".to_string();
    println!("{}", max_freq_sum(s));
}

fn max_freq_sum(s: String) -> i32 {
    let mut hashmap = HashMap::new();
    for ch in s.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    let (mut vowel_max, mut cons_max) = (0, 0);
    for (key, val) in hashmap.iter() {
        if vec!['a', 'e', 'i', 'o', 'u'].contains(&key) {
            vowel_max = vowel_max.max(*val);
        } else {
            cons_max = cons_max.max(*val);
        }
    }
    return vowel_max + cons_max;
}