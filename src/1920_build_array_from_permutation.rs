fn main() {
    let input = vec![0, 2, 1, 5, 3, 4];
    println!("{:?}", build_array(input));
}

fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = nums.clone();
    for idx in 0..nums.len() {
        ans[idx] = nums[nums[idx] as usize];
    }
    return ans;
}