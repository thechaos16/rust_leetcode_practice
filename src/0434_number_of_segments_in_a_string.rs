fn main() {
    let s = "    Hello,     my name is John a  ".to_string();
    println!("{}", count_segments(s));
}

fn count_segments(s: String) -> i32 {
    let mut cnt = 0;
    let mut last_empty = true;
    for ch in s.chars() {
        if ch == ' ' {
            last_empty = true;
        } else {
            if last_empty {
                cnt += 1;
                last_empty = false;
            }
        }
    }
    return cnt;
}