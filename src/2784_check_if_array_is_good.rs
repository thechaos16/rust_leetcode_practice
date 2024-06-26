use std::collections::HashMap;


fn main() {
    let nums = vec![2, 1, 3];
    println!("{}", is_good(nums));
}

fn is_good(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::new();
    for num in nums.iter() {
        *hashmap.entry(num).or_insert(0) += 1;
    }
    for idx in 1..(nums.len() - 1) {
        if *hashmap.get(&(idx as i32)).unwrap() != 1 {
            return false;
        }
    }
    return *hashmap.get(&(nums.len() as i32 - 1)).unwrap() == 2;
}