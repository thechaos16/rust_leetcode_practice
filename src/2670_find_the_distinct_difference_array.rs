use std::collections::HashMap;


fn main() {
    let nums = vec![3,2,3,4,2];
    println!("{:?}", distinct_difference_array(nums))
}

fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::new();
    let mut new_hashmap = HashMap::new();
    for num in nums.iter() {
        *hashmap.entry(num).or_insert(0) += 1;
    }
    let mut ans = vec![];
    for num in nums.iter() {
        *hashmap.entry(num).or_insert(0) -= 1;
        if *hashmap.get(num).unwrap() == 0 {
            hashmap.remove(num);
        }
        *new_hashmap.entry(num).or_insert(0) += 1;
        ans.push(new_hashmap.len() as i32 - hashmap.len() as i32);
    }
    return ans;
}