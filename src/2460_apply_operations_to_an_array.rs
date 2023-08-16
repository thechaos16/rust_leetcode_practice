fn main () {
    let input = vec![5,119,119,119,119,218,0,0,0,0];
    println!("{:?}", apply_operations(input));
}

fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut copied = nums.clone();
    let mut new_vec = vec![];
    for idx in 0..nums.len() - 1 {
        if copied[idx] != 0 && copied[idx] == copied[idx + 1] {
            copied[idx] *= 2;
            copied[idx + 1] = 0;
        }
        new_vec.push(0);
    }
    new_vec.push(0);
    let mut cur_idx = 0;
    for idx in 0..nums.len() {
        if copied[idx] != 0 {
            new_vec[cur_idx] = copied[idx];
            cur_idx += 1;
        }
    }
    return new_vec;
}