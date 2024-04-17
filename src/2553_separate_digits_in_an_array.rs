fn main() {
    let num = vec![13, 25, 83, 77];
    println!("{:?}", separate_digits(num));
}

fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for num_idx in 0..nums.len() {
        let mut mut_num = nums[num_idx];
        let mut temp_vec = vec![];
        while mut_num > 0 {
            temp_vec.push(mut_num % 10);
            mut_num /= 10;
        }
        for idx in (0..temp_vec.len()).rev() {
            ans.push(temp_vec[idx]);
        }
    }
    return ans;
}