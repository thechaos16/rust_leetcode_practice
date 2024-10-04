fn main() {
    let nums = vec![0, 1, 1, 0];
    println!("{:?}", get_sneaky_numbers(nums));
}

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut count_vec = vec![];
    let mut ans = vec![];
    for _ in 0..(nums.len() - 2) {
        count_vec.push(0);
    }
    for num in nums {
        count_vec[num as usize] += 1;
        if count_vec[num as usize] == 2 {
            ans.push(num);
        }
    }
    return ans
}