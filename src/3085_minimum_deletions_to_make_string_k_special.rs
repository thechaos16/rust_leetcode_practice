use std::collections::HashMap;


fn main() {
    let word = "dabdcbdcdcd".to_string();
    let k = 2;
    println!("{}", minimum_differences(word, k));
}

fn minimum_differences(word: String, k: i32) -> i32 {
    let mut max_ans = word.len() as i32;
    let mut already_del = 0;
    let mut hashmap = HashMap::new();
    for ch in word.chars() {
        *hashmap.entry(ch).or_insert(0) += 1;
    }
    let mut sorted_cnt: Vec<_> = hashmap.into_iter().collect();
    sorted_cnt.sort_by(|x, y| x.1.cmp(&y.1));

    for (_, val) in &sorted_cnt {
        let mut cur_del = 0;
        for (_, val2) in &sorted_cnt {
            cur_del += (val2 - val - k).max(0);
        }
        max_ans = max_ans.min(cur_del + already_del);
        already_del += val;
    }
    return max_ans;
}