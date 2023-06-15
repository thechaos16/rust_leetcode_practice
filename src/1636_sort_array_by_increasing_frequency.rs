use std::collections::HashMap;
use std::cmp::Ordering;

fn main () {
    let num = vec![1, 1, 2, 2, 2, 3];
    println!("{:?}", frequency_sort(num));
}

fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut count = HashMap::new();
    for num in nums {
        let cnt = count.entry(num).or_insert(0);
        *cnt += 1;
    }

    let mut counter_vec: Vec<(&i32, &u32)> = count.iter().collect();
    counter_vec.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Greater
        } else if a.1 < b.1 {
            Ordering::Less
        } else {
            if a.0 < b.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });
    let mut res = vec![];
    for idx in 0..counter_vec.len() {
        for _ in 0..counter_vec[idx].1.clone() {
            res.push(counter_vec[idx].0.clone());
        }
    }
    return res;
}