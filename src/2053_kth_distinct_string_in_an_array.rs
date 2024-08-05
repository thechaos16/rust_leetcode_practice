use std::collections::HashMap;


fn main() {
    let arr = vec!["d".to_string(), "b".to_string(), "c".to_string(), "b".to_string(), "c".to_string(), "a".to_string()];
    let k = 2;
    println!("{}", kth_distinct(arr, k));
}

fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut hashmap = HashMap::new();
    for string in arr.clone().into_iter() {
        *hashmap.entry(string).or_insert(0) += 1;
    }
    let mut cnt = 0;
    for string in arr.into_iter() {
        if *hashmap.get(&string).unwrap() == 1 {
            cnt += 1;
            if cnt == k {
                return string;
            }
        }
    }
    return "".to_string();
}