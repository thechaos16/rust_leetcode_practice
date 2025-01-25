fn main() {
    let s = "leetcode".to_string();
    println!("{}", max_power(s));
}

fn max_power(s: String) -> i32 {
    let (_, cnt, max) = s.chars().fold(('.', 0, 0), | (last_ch, cnt, max), ch | {
        if ch == last_ch {
            (last_ch, cnt + 1, max)
        } else {
            (ch, 1, max.max(cnt))
        }
    });
    cnt.max(max)
}
