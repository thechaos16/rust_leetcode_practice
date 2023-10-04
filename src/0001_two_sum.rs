use std::collections::HashMap;


fn main() {
    let nums1 = [2,7,11,15];
    let target1: i32 = 9;
    let nums2 = [3,2,4];
    let target2: i32 = 6;

    println!("{:?}", two_sum((&nums1).to_vec(), target1));
    println!("{:?}", two_sum((&nums2).to_vec(), target2));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_idx) = hash_map.get(&complement) {
            return vec![complement_idx as i32, idx as i32];
        }
        hash_map.insert(num, idx);
    }
    vec![]
}

