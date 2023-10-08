use std::collections::HashMap;


fn main() {
    let inputs = vec![vec![3,1,2,4,5], vec![1,2,3,4], vec![3,4,5,6]];
    println!("{:?}", intersection(inputs));
}

fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cur_map = HashMap::new();
    let mut ans = vec![];
    for idx in 0..nums.len() {
        for idx2 in 0..nums[idx].len() {
            *cur_map.entry(nums[idx][idx2]).or_insert(0) += 1;
        }
    }
    for (key, value) in cur_map {
        if value == nums.len() {
            ans.push(key);
        }
    }
    ans.sort();
    return ans;
}