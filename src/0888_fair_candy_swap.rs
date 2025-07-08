use std::collections::HashSet;


fn main() {
    let alice_sizes = vec![1, 1];
    let bob_sizes = vec![2, 2];
    println!("{:?}", fair_candy_swap(alice_sizes, bob_sizes));
}

fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let a_sum: i32 = alice_sizes.iter().sum();
    let b_sum: i32 = bob_sizes.iter().sum();
    let b_hash: HashSet<i32> = bob_sizes.iter().cloned().collect();
    for alice in alice_sizes {
        if b_hash.contains(&(alice + (b_sum - a_sum) / 2)) {
            return vec![alice, alice + (b_sum - a_sum) / 2];
        }
    }
    return vec![];
}