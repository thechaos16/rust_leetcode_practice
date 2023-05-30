use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let input1 = vec![1, 2, 2, 1, 3, 4, 5, 5, 5];
    let input2 = vec![2, 2, 3];
    println!("{:?}", intersection(input1, input2));
}

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let hash_set1: HashSet<i32> = HashSet::from_iter(nums1);
    let hast_set2: HashSet<i32> = HashSet::from_iter(nums2);
    let intersection = hash_set1.intersection(&hast_set2);
    let vector = Vec::from_iter(intersection);
    return vector.into_iter().cloned().collect();
}