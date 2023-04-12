
fn main(){
    let mut input: Vec<i32> = [1, 3, 5, 5, 7, 9].to_vec();
    let mut input2: Vec<i32> = [1, 1, 1, 1,2].to_vec();
    let mut input3: Vec<i32> = [1, 5, 5, 5, 7, 8, 9, 9, 9].to_vec();
    println!("{}", remove_duplicates(&mut input));
    println!("{:?}", input);
    println!("{}", remove_duplicates(&mut input2));
    println!("{:?}", input2);
    println!("{}", remove_duplicates(&mut input3));
    println!("{:?}", input3);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut next_idx: i32 = 1;
    for idx in 1..nums.len(){
        if nums[idx] == nums[idx - 1]{
            continue;
        } else {
            nums[next_idx as usize] = nums[idx];
            next_idx += 1;
        }
    }
    return next_idx;
}