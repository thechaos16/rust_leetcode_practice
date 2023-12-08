use std::collections::HashMap;


fn main() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();
    println!("{}", num_jewels_in_stones(jewels, stones));
}

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut counter = HashMap::new();
    for stone in stones.chars() {
        let before_count = counter.entry(stone).or_insert(0);
        *before_count += 1;
    }
    let mut ans = 0;
    for jewel in jewels.chars() {
        if counter.contains_key(&jewel) {
            ans += counter.get(&jewel).unwrap();
        }
    }
    return ans;
}