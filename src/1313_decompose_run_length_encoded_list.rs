fn main () {
    let input = vec![1, 2, 3, 4];
    println!("{:?}", decompress_rl_elist(input));
}

fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for idx in 0..nums.len() / 2 {
        let freq = nums[idx * 2];
        let number = nums[idx * 2 + 1];
        for idx2 in 0..freq {
            result.push(number);
        }
    }
    return result;
}