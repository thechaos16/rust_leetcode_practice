fn main() {
    let nums = vec![7, 1, 5, 4];
    println!("{}", maximum_difference(nums));
}

fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut ans = -1;
    let mut min_so_far = nums[0];
    for num in nums {
        min_so_far = min_so_far.min(num);
        ans = ans.max(num - min_so_far);
    }
    if ans > 0 {
        return ans;
    } else {
        return -1;
    }
}