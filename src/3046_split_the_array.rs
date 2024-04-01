use std::collections::HashMap;


fn main() {
    let nums = vec![1, 1, 2, 2, 3, 4];
    println!("{}", is_possible_to_split(nums));
}

fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::new();
    let mut max_val = 0;
    for num in nums.iter() {
        *hashmap.entry(num).or_insert(0) += 1;
        if *hashmap.get(num).clone().unwrap() > max_val {
            max_val = *hashmap.get(num).unwrap();
        }
        if max_val > 2 {
            return false;
        }
    }
    return true;
}