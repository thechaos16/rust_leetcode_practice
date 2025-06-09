fn main() {
    let nums = vec![4, 3, 2, 1];
    println!("{:?}", transform_array(nums));
}

fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let mut even_cnt = 0 as usize; 
    let mut ans = vec![0; nums.len()];
    for num in nums.iter() {
        if num % 2 == 0 {
            even_cnt += 1;
        }
    }
    for idx in even_cnt..nums.len() {
        ans[idx] = 1;
    }
    return ans
}