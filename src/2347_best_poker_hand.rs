use std::collections::HashMap;

fn main() {
    let ranks = vec![13, 2, 3, 13, 9];
    let suits = vec!["a".to_string(), "a".to_string(), "a".to_string(), "c".to_string(), "a".to_string()];
    println!("{}", best_hand(ranks, suits));
}

fn best_hand(ranks: Vec<i32>, suits: Vec<String>) -> String {
    let mut flush = true;
    for idx in 1..suits.len() {
        if suits[0] != suits[idx] {
            flush = false;
            break;
        }
    }
    if flush {
        return "Flush".to_string();
    }
    let mut hashmap = HashMap::new();
    let mut max_val = 0;
    for rank in ranks {
        let before_entry = hashmap.entry(rank).or_insert(0);
        *before_entry += 1;
        if *before_entry > max_val {
            max_val = *before_entry;
        }
    }
    match max_val {
        4 => "Three of a Kind".to_string(),
        3 => "Three of a Kind".to_string(),
        2 => "Pair".to_string(),
        _ => "High Card".to_string()
    }
}