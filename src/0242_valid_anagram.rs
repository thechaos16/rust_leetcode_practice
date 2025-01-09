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
    let mut char_counts: HashMap<char, i32> = HashMap::new();

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        *char_counts.entry(s_char).or_default() += 1;
        *char_counts.entry(t_char).or_default() -= 1;
    }
    char_counts.values().all(|&count| count == 0)
}