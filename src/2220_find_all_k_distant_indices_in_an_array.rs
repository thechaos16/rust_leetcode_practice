fn main() {
    let nums = vec![3, 4, 9, 1, 3, 9, 5];
    let key = 9;
    let k = 1;
    println!("{:?}", find_k_distant_indices(nums, key, k));
}

fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut ans = vec![];
    let mut key_indices = vec![];
    for idx in 0..nums.len() {
        if nums[idx] == key {
            key_indices.push(idx as i32);
        }
    }
    for idx in 0..nums.len() {
        for key_idx in key_indices.iter() {
            if (idx as i32 - key_idx).abs()<= k {
                ans.push(idx as i32);
                break;
            }
        }
    }
    return ans;
}