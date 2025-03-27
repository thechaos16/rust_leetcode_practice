use std::collections::HashMap;


fn main() {
    let nums = vec![1, 2, 2, 2];
    println!("{}", minimum_index(nums));
}

fn minimum_index(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    for num in nums.iter() {
        *hashmap.entry(num).or_insert(0) += 1;
    }
    let (mut number, mut count) = (0, 0);
    for (key, value) in hashmap.into_iter() {
        if value * 2 > nums.len() as i32 {
            number = *key;
            count = value;
            break
        }
    }
    let mut sofar = 0;
    for idx in 0..(nums.len() - 1) {
        if nums[idx] == number {
            sofar += 1;
        }
        if sofar * 2 > (idx + 1) as i32 && (count - sofar) * 2 > (nums.len() - idx - 1) as i32 {
            return idx as i32;
        }
    }
    return -1;
}