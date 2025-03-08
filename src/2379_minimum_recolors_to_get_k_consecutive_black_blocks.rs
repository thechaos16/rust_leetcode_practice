use std::collections::HashMap;


fn main() {
    let blocks = "WBBWWBBWBW".to_string();
    let k = 7;
    println!("{}", minimum_recolors(blocks, k));
}

fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let mut ans = 999999;
    let mut hashmap = HashMap::new();
    hashmap.insert('W', 0);
    hashmap.insert('B', 0);
    for idx in 0..blocks.len() {
        *hashmap.entry(blocks.chars().nth(idx).unwrap()).or_insert(0) += 1;
        if idx >= k as usize {
            *hashmap.entry(blocks.chars().nth(idx - k as usize).unwrap()).or_insert(0) -= 1;
        }
        if idx >= (k - 1) as usize {
            let temp = *hashmap.entry('W').or_insert(0);
            if temp < ans {
                ans = temp;
            }
        }
    }
    return ans;
}