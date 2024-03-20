use std::collections::HashMap;


fn main() {
    let input1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let input2 = vec![vec![3, 1], vec![1, 5]];
    println!("{:?}", merge_similar_items(input1, input2));
}

fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hash = HashMap::new();
    for item in items1.iter() {
        let before_val = hash.entry(item[0]).or_insert(0);
        *before_val += item[1];
    }
    for item in items2.iter() {
        let before_val = hash.entry(item[0]).or_insert(0);
        *before_val += item[1];
    }
    let mut v: Vec<_> = hash.clone().into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    let mut ans = vec![];
    for row in v.iter() {
        ans.push(vec![row.0, row.1]);
    }
    return ans;
}