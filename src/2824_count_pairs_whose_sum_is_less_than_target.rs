fn main() {
    let nums = vec![-1, 1, 2, 3, 1];
    let target = 2;
    println!("{}", count_pairs(nums, target));
}

fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut mut_nums = nums;
    mut_nums.sort();
    let mut cnt = 0;
    for idx in 0..mut_nums.len() {
        for idx2 in (idx + 1)..mut_nums.len() {
            if mut_nums[idx] + mut_nums[idx2] < target {
                cnt += 1;
            } else {
                break;
            }
        }
    }
    return cnt;
}