use std::collections::HashMap;


fn main() {
    let input1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let input2 = vec![vec![3, 1], vec![1, 5]];
    println!("{:?}", merge_similar_items(input1, input2));
}

fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hash = HashMap::new();
    for item in items1.iter() {
        *hash.entry(item[0]).or_insert(0) += item[1];
    }
    for item in items2.iter() {
        *hash.entry(item[0]).or_insert(0) += item[1];
    }
    let mut sorted_items: Vec<_> = hash.clone().into_iter().collect();
    sorted_items.sort_by_key(|&k| k.0);
    return sorted_items.into_iter().map(|(k, v)| vec![k, v]).collect();
}
