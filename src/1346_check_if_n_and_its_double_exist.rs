use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let input = vec![11, 2, 3, 5];
    println!("{}", check_if_exist(input));
}

fn check_if_exist(arr: Vec<i32>) -> bool {
    let hashset: HashSet<i32> = HashSet::from_iter(arr.clone());
    let mut zero_cnt = 0;
    for val in arr {
        if val == 0 {
            zero_cnt += 1;
        }
    }
    if zero_cnt >= 2 {
        return true;
    }
    for val in &hashset {
        if *val == 0 {
            continue;
        }
        if hashset.contains(&(val * 2)) {
            return true;
        }
    }
    return false;
}