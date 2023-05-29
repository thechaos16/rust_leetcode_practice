use std::collections::HashMap;


fn main () {
    let input = vec![1, 3, 2, 1, 3, 2, 2];
    println!("{:?}", number_of_pairs(input));
}

fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut counter = HashMap::new();
    for num in nums {
        let number = counter.entry(num).or_insert(0);
        *number += 1;
    }
    let counter_vec: Vec<(&i32, &u32)> = counter.iter().collect();
    let mut pair = 0;
    let mut remain = 0;
    for (_a, b) in counter_vec {
        pair += b / 2;
        remain += b % 2;
    }
    return vec![pair as i32, remain as i32];
}