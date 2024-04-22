fn main() {
    let s = "hello".to_string();
    println!("{}", score_of_string(s));
}

fn score_of_string(s: String) -> i32 {
    let mut cnt = 0;
    for idx in 0..(s.len() - 1) {
        let first = s.chars().nth(idx).unwrap() as u8;
        let second = s.chars().nth(idx + 1).unwrap() as u8;
        cnt += (first as i32 - second as i32).abs();
    }
    return cnt;
}
