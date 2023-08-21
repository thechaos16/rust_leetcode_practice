fn main () {
    let input = vec![1, 2, 3, 3, 4];
    println!("{}", contains_duplicate(input));
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_vec = nums;
    num_vec.sort();
    for idx in 0..num_vec.len() - 1 {
        if num_vec[idx] == num_vec[idx + 1] {
            return true;
        }
    }
    return false;
}