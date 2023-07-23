fn main() {
    let input = vec![3, 2, 1, 4];
    println!("{}", find_non_min_or_max(input));
}

fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return -1;
    }
    let mut min_val = 100;
    let mut max_val = 1;
    for num in nums.clone().into_iter() {
        if num < min_val {
            min_val = num;
        }
        if num > max_val  {
            max_val = num;
        }
    }
    for num in nums.clone().into_iter() {
        if num != min_val && num != max_val {
            return num;
        }
    }
    return -1;
}