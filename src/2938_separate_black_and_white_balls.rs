fn main() {
    let s = "101".to_string();
    println!("{}", minimum_steps(s));
}

fn minimum_steps(s: String) -> i64 {
    let mut ans = 0;
    let mut one_cnt = 0;
    for idx in (0..s.len()).rev() {
        if s.chars().nth(idx).unwrap() == '1' {  
            ans += s.len() as i32 - 1 - one_cnt - idx as i32;
            one_cnt += 1;
        }
    }
    return ans as i64
}