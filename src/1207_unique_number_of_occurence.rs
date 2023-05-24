use std::collections::HashMap;

fn main () {
    let input = vec![1,2,2,1,1,3];
    println!("{}", unique_occurences(input));
    let input2 = vec![1,2,2,1,1,3,3];
    println!("{}", unique_occurences(input2));
}

fn unique_occurences(arr: Vec<i32>) -> bool {
    let mut count = HashMap::new();
    for num in arr {
        let number = count.entry(num).or_insert(0);
        *number += 1;
    }
    let mut counter_vec: Vec<(&i32, &u32)> = count.iter().collect();
    counter_vec.sort_by(|a, b| b.1.cmp(a.1));
    for idx in 0..counter_vec.len() - 1 {
        if counter_vec[idx].1.clone() == counter_vec[idx + 1].1.clone() {
            return false;
        }
    }
    return true;
}