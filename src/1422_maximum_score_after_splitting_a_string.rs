fn main() {
    let input = "011101".to_string();
    println!("{}", max_score(input));
}

fn max_score(s: String) -> i32 {
    let mut one_cnt = 0;
    let mut max_val = 0;
    let mut last_zero = 0;
    for ch in s.chars() {
        if ch == '1' {
            one_cnt += 1;
        }
    }
    for idx in 0..s.len() {
        if s.chars().nth(idx).unwrap() == '1' {
            one_cnt -= 1;
        } else {
            last_zero += 1;
        }
        if one_cnt + last_zero > max_val {
            max_val = one_cnt + last_zero;
        }        
    }
    return max_val;
}