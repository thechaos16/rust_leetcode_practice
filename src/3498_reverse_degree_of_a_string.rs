fn main() {
    let s = "abc".to_string();
    println!("{}", reverse_degree(s));
}

fn reverse_degree(s: String) -> i32 {
    let mut ans = 0;
    for idx in 0..s.len() {
        ans += (idx as i32 + 1) * (26 - s.chars().nth(idx).unwrap() as i32 + 'a' as i32);
    }
    ans
}