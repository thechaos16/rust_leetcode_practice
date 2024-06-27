use std::collections::HashMap;


fn main() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
    println!("{}", find_center(edges));
}

fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut max_node = 0;
    for edge in edges.iter() {
        for node in edge.iter() {
            *hashmap.entry(node).or_insert(0) += 1;
            if max_node < *node {
                max_node = *node;
            }
        }
    }
    for (key, value) in hashmap {
        if value == max_node - 1 {
            return *key;
        }
    }
    return 0;
}