fn main() {
    let score = vec![5, 4, 3, 2, 1, 99, 12, 23];
    println!("{:?}", find_relative_ranks(score));
}

fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut sorted_indices: Vec<usize> = (0..score.len()).collect();
    sorted_indices.sort_by_key(|&idx| -score[idx]);

    let mut result = vec![String::new(); score.len()];
    for (rank, &idx) in sorted_indices.iter().enumerate() {
        result[idx] = match rank {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => (rank + 1).to_string(),
        };
    }

    result
}