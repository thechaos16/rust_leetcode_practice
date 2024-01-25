use std::collections::HashMap;


fn main() {
    let input = vec![1, 2, 3, 4, 4, 3, 1, 2];
    println!("{}", has_groups_size_x(input));
}

fn has_groups_size_x(deck: Vec<i32>) -> bool {
    if deck.len() == 1 {
        return false;
    }
    let mut hashmap = HashMap::new();
    for card in deck {
        let before_cnt = hashmap.entry(card).or_insert(0);
        *before_cnt += 1;
    }
    let counter_vec: Vec<(&i32, &u32)> = hashmap.iter().collect();
    for idx in 0..counter_vec.len() {
        for idx2 in (idx + 1)..counter_vec.len() {
            if counter_vec[idx].1 >= counter_vec[idx2].1 {
                if gcd(*counter_vec[idx].1 as i32, *counter_vec[idx2].1 as i32) == 1 {
                    return false;
                }
            } else {
                if gcd(*counter_vec[idx2].1 as i32, *counter_vec[idx].1 as i32) == 1 {
                    return false;
                }
            }
        }
    }
    return true;
}

fn gcd(m: i32, n: i32) -> i32 {
    if m % n == 0 {
        return n;
    } else {
        return gcd(n, m % n);
    }
}