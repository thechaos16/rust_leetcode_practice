use std::collections::HashMap;


fn main() {
    let nums = vec![3, 2, 3, 2, 2, 2];
    println!("{}", divide_array(nums));
}

fn divide_array(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::new();
    for num in nums {
        *hashmap.entry(num).or_insert(0) += 1;
    }
    for (_num, count) in hashmap.into_iter() {
        if count % 2 == 1 {
            return false;
        }
    }
    return true;
}