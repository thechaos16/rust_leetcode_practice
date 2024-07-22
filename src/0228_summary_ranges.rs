fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    println!("{:?}", summary_ranges(nums));
}

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }
    let mut start = nums[0];
    let mut end = nums[0];
    let mut ans = vec![];
    for idx in 1..nums.len() {
        if nums[idx] == end + 1 {
            end = nums[idx];
        } else {
            if start == end {
                ans.push(format!("{}", start));
            } else {
                ans.push(format!("{}->{}", start, end));
            }
            start = nums[idx];
            end = nums[idx];
        }
    }
    if start == end {
        ans.push(format!("{}", start));
    } else {
        ans.push(format!("{}->{}", start, end));
    }
    return ans;
}