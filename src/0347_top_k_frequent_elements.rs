use std::collections::HashMap;


fn main() {
    let input = vec![1, 1, 1, 2, 2, 3,4,4,4,4,4,4,5,5,6,6,6,6,7,7,7];
    let k = 4;
    println!("{:?}", top_k_frequent(input, k));
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = HashMap::new();
    for idx in 0..nums.len() {
        let before_cnt = count.entry(nums[idx]).or_insert(0);
        *before_cnt += 1;
    }

    let mut counter_vec: Vec<(&i32, &u32)> = count.iter().collect();
    counter_vec.sort_by(|a, b| a.1.cmp(b.1));
    let mut res = vec![];
    for idx in 0..k as usize {
        res.push(counter_vec[idx].0.clone());
    }
    return res;
}
