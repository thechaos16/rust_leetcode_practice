use std::cmp;


fn main() {
    let words = vec!["hello".to_string(), "i".to_string(), "am".to_string(), "leetcode".to_string(), "hello".to_string()];
    let target = "hello".to_string();
    let start_index = 1;
    println!("{}", closest_target(words, target, start_index));
}

fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let mut ans = words.len() as i32;
    for idx in 0..words.len() {
        if words[idx] == target {
            if start_index == idx as i32 {
                return 0;
            }
            let diff = (start_index - idx as i32).abs();
            ans = cmp::min(ans, cmp::min(diff, words.len() as i32 - diff));
        }
    }
    if ans == words.len() as i32 {
        return -1;
    }
    return ans;
}