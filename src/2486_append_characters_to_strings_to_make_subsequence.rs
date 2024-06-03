fn main() {
   let s = "coaching".to_string();
   let t = "coding".to_string();
   println!("{}", append_characters(s, t));
}

fn append_characters(s: String, t: String) -> i32 {
    let mut t_chars = t.chars().peekable();
    for ch in s.chars() {
        if let Some(&t_ch) = t_chars.peek() {
            if ch == t_ch {
                t_chars.next();
            }
        }
    }
    t_chars.count() as i32
}
