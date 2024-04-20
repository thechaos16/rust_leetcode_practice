fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("{:?}", running_sum(nums));
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut cur_sum = 0;
    let mut ans = vec![];
    for num in nums.iter() {
        cur_sum += num;
        ans.push(cur_sum);
    }
    return ans;
}