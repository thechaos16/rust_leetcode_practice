fn main() {
    let nums = vec![4, 5, 4];
    println!("{:?}", sort_by_reflection(nums));
}

fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
    let mut reverse_binary = vec![];
    for num in nums.clone().into_iter() {
        let one_bin: String = format!("{:b}", num).chars().rev().collect();
        reverse_binary.push(i32::from_str_radix(&one_bin, 2).unwrap());
    }
    let mut combined: Vec<_> = nums.iter().zip(reverse_binary.iter()).collect();
    combined.sort_unstable_by_key(|&(num, key)| (key, num));
    let mut ans = vec![];
    for idx in 0..combined.len() {
        ans.push(*combined[idx].0);
    }
    return ans;
}