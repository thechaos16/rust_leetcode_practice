use std::collections::HashMap;
use std::cmp::Ordering;


fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    println!("{:?}", relative_sort_array(arr1, arr2));
}

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let position_map: HashMap<_, _> = arr2.iter().enumerate().map(|(idx, &num)| (num, idx)).collect();
    let mut arr1_mut = arr1;
    arr1_mut.sort_by(|&a, &b| {
        match (position_map.get(&a), position_map.get(&b)) {
            (Some(&pos_a), Some(&pos_b)) => pos_a.cmp(&pos_b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.cmp(&b),
        }
    });
    arr1_mut
}