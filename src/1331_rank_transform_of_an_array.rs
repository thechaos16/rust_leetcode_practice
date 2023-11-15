use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;


fn main() {
    let input = vec![40,10,20,30];
    println!("{:?}", array_rank_transform(input));
}

fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let hash: HashSet<i32> = HashSet::from_iter(arr.clone());
    let mut copy: Vec<_> = hash.into_iter().collect();
    copy.sort();
    let mut hashmap = HashMap::new();
    for idx in 0..copy.len() {
        hashmap.insert(copy[idx], (idx + 1) as i32);
    }
    for idx in 0..arr.len() {
        res.push(*hashmap.get(&arr[idx]).unwrap());
    }
    return res;
}