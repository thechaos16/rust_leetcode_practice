use std::collections::HashMap;


fn main() {
    let nums = vec![1,2,3,1,1,3];
    println!("{}", num_identical_pairs(nums));
}

fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    for num in nums.iter() {
        *(hashmap.entry(num).or_insert(0)) += 1;
    }
    let mut ans = 0;
    for (key, val) in hashmap {
        ans += (val * (val - 1)) / 2;
    }
    return ans;
}