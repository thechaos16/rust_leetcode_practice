fn main() {
    let word = "abbcccc".to_string();
    println!("{}", possible_string_count(word));
}

fn possible_string_count(word: String) -> i32 {
    let mut last_str = ' ';
    let mut last_cnt = 0;
    let mut ans = 1;
    for ch in word.chars() {
        if ch == last_str {
            last_cnt += 1;
        } else {
            ans += 0.max(last_cnt - 1);
            last_str = ch;
            last_cnt = 1;
        }
    }
    return ans + 0.max(last_cnt - 1);
}