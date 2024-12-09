fn main() {
    let nums = vec![3, 4, 1, 2, 6];
    let queries = vec![vec![0, 4]];
    println!("{:?}", is_array_special(nums, queries));
}

fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut cur_parity = 0;
    let mut parity_vec = vec![0];
    for idx in 0..(nums.len() - 1) {
        let cur_val = (nums[idx] + nums[idx + 1]) % 2;
        cur_parity += cur_val;
        parity_vec.push(cur_parity);
    }
    let mut ans = vec![];
    for query in queries {
        if parity_vec[query[1] as usize] - parity_vec[query[0] as usize] == query[1] - query[0] {
            ans.push(true);
        } else {
            ans.push(false);
        }
    }
    return ans;
}