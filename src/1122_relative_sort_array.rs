use std::collections::HashMap;
use std::cmp::Ordering;


fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    println!("{:?}", relative_sort_array(arr1, arr2));
}

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut arr2_hashmap = HashMap::<i32, usize>::new();
    for idx in 0..arr2.len() {
        arr2_hashmap.insert(arr2[idx], idx);
    }
    let mut arr1_mut = arr1;
    arr1_mut.sort_by(|a, b| {
        if arr2_hashmap.contains_key(&a) && arr2_hashmap.contains_key(&b) {
            if arr2_hashmap.get(a) > arr2_hashmap.get(b) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if arr2_hashmap.contains_key(&a) && !arr2_hashmap.contains_key(&b) {
            Ordering::Less
        } else if !arr2_hashmap.contains_key(&a) && arr2_hashmap.contains_key(&b) {
            Ordering::Greater
        } else {
            if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });
    return arr1_mut;
}