use std::collections::HashMap;


fn main() {
    let nums = vec![2, 3, 4, 6];
    println!("{}", tuple_same_product(nums));
}

fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut hashmap = HashMap::new();
    for idx in 0..nums.len() {
        for idx2 in (idx + 1)..nums.len() {
            let product = nums[idx] * nums[idx2];
            let before_cnt = hashmap.entry(product).or_insert(0);
            cnt += *before_cnt * 8;
            *before_cnt += 1;
        }
    }
    return cnt;
}