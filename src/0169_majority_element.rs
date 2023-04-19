use std::collections::HashMap;

fn main() {
    let input = vec![3, 2, 3];
    let input2 = vec![2,2,1,1,1,2,2];
    println!("{}", get_major_elem(input));
    println!("{}", get_major_elem(input2));
}

fn get_major_elem(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    let mut ans = 0;
    for idx in 0..nums.len() {
        let mut val = 0;
        if count.contains_key(&Some(nums[idx]).unwrap()) { 
            val = *count.get(&nums[idx]).unwrap_or(&0);
        }
        if val + 1 >= (nums.len() + 1) / 2 {
            ans = nums[idx];
            break;
        }
        count.insert(nums[idx], val + 1);
    }
    return ans;
}