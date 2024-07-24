use std::cmp;

fn main() {
    let s1 = "abc".to_string();
    let s2 = "abb".to_string();
    let s3 = "ab".to_string();
    println!("{}", find_minimum_operations(s1, s2, s3));
}

fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let min_len = cmp::min(s1.len(), cmp::min(s2.len(), s3.len()));
    let mut ans = 0;
    for idx in 0..min_len {
        if s1.chars().nth(idx) == s2.chars().nth(idx) && s1.chars().nth(idx) == s3.chars().nth(idx) {
            ans = idx + 1;
        } else {
            break;
        }
    }
    if ans == 0 {
        return -1;
    }
    return s1.len() as i32 + s2.len() as i32 + s3.len() as i32 - 3 * ans as i32;
}