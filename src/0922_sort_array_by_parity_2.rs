fn main() {
    let input = vec![4, 2, 5, 7];
    println!("{:?}", sort_array_by_parity_ii(input));
}

fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for _idx in 0..nums.len() {
        ans.push(0);
    }
    let mut odd_idx = 0;
    let mut even_idx = 0;
    for num in nums {
        if num % 2 == 0 {
            ans[even_idx * 2] = num;
            even_idx += 1;
        } else {
            ans[odd_idx * 2 + 1] = num;
            odd_idx += 1;
        }
    }
    return ans;
}