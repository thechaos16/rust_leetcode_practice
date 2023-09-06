use std::collections::HashMap;


fn main () {
    let input = vec![1, 2, 2, 3, 1, 3, 2, 2, 3, 3, 3, 3];
    println!("{}", find_shortest_sub_array(input));
}

fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    let mut min_idx:HashMap<i32, i32> = HashMap::new();
    let mut max_idx:HashMap<i32, i32> = HashMap::new();
    let mut max_val = 0;
    let mut max_elem = vec![];
    for idx in 0..nums.len() {
        let before_cnt = hash.entry(nums[idx]).or_insert(0);
        *before_cnt += 1;
        if *before_cnt > max_val {
            max_val = *before_cnt;
            max_elem = vec![nums[idx]];
        } else if *before_cnt == max_val {
            max_elem.push(nums[idx]);
        }
        if !min_idx.contains_key(&nums[idx]) {
            min_idx.insert(nums[idx], idx as i32);
        }
        max_idx.insert(nums[idx], idx as i32);
    }
    let mut answer = nums.len() as i32;
    for idx in 0..max_elem.len() {
        let cur = max_idx[&max_elem[idx]] - min_idx[&max_elem[idx]] + 1;
        if cur < answer {
            answer = cur;
        }
    }
    return answer;
}