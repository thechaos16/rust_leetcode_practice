use std::collections::HashMap;


fn main() {
    let score = vec![5, 4, 3, 2, 1, 99, 12, 23];
    println!("{:?}", find_relative_ranks(score));
}

fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut score_ = score.clone();
    score_.sort_by(|a, b| b.cmp(a));
    let mut hashmap = HashMap::new();
    for idx in 0..score_.len() {
        hashmap.insert(score_[idx], convert_name(idx + 1));
    }
    let mut result = vec![];
    for idx in 0..score.len() {
        result.push(hashmap.get(&score[idx]).unwrap().to_string());
    }
    return result;
}

fn convert_name(rank: usize) -> String {
    match rank {
        1 => return "Gold Medal".to_string(),
        2 => return "Silver Medal".to_string(),
        3 => return "Bronze Medal".to_string(),
        _ => return format!("{}", rank)
    }
}