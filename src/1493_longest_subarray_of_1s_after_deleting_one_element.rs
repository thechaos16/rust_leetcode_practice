use std::cmp;


fn main() {
    let input = vec![1, 1, 1, 0, 1, 1];
    println!("{}", longest_subarray(input));
}

fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut last = nums[0];
    let mut count = 1;
    let mut aggr: Vec<i32> = vec![];
    for idx in 1..nums.len() {
        if nums[idx] != last {
            if last == 0 {
                aggr.push(-count);
            } else {
                aggr.push(count);
            }
            count = 1;
            last = nums[idx]
        } else {
            count += 1;
        }
    }
    if last == 0 {
        aggr.push(-count);
    } else {
        aggr.push(count);
    }
    if aggr.len() == 1 {
        if aggr[0] > 0 {
            return aggr[0] - 1;
        } else {
            return 0;
        }
    }
    let mut max_val = 0;
    for idx in 0..aggr.len() {
        max_val = cmp::max(max_val, aggr[idx]);
        if idx < aggr.len() - 2 && aggr[idx] > 0 {
            if aggr[idx + 1] == -1 {
                max_val = cmp::max(max_val, aggr[idx] + aggr[idx + 2]);
            }
        }
    }
    return max_val;
}