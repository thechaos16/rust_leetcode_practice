use std::collections::HashMap;


fn main() {
    let nums = vec![1,2,2,3,1,4];
    println!("{}", max_frequency_elements(nums));
}

fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut max_val = 0;
    for num in nums.into_iter() {
        let cur_entry = hashmap.entry(num).or_insert(0);
        *cur_entry += 1;
        if max_val < *cur_entry {
            max_val = *cur_entry;
        }
    }
    let mut ans = 0;
    for (key, val) in hashmap.into_iter() {
        if val == max_val {
            ans += val;
        }
    }
    return ans;
}