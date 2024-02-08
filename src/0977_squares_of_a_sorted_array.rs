fn main() {
    let input = vec![-4, -1, 0, 10];
    println!("{:?}", sorted_squares(input));
}

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for val in nums {
        ans.push(val * val);
    }
    ans.sort();
    return ans;
}