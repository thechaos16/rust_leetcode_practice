use std::collections::HashSet;
use std::iter::FromIterator;

fn main (){
    let input1 = vec![1, 2, 3];
    let input2 = vec![2, 4, 6];
    println!("{:?}", find_difference(input1, input2));
}

fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diff1 = vec![];
    let mut diff2 = vec![];
    let hash1: HashSet<i32,> = HashSet::from_iter(nums1);
    let hash2: HashSet<i32,> = HashSet::from_iter(nums2);
    for num in &hash1 {
        if !hash2.contains(&num) {
            diff1.push(*num);
        }
    }
    for num in &hash2 {
        if !hash1.contains(&num) {
            diff2.push(*num);
        }
    }
    return vec![diff1, diff2];
}