use std::cmp;


fn main() {
    let num_ones = 2;
    let num_zeros = 3;
    let num_neg_ones = 0;
    let k = 2;
    println!("{}", k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k));
}

fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
    if num_ones >= k {
        cmp::min(num_ones, k)
    } else if (num_ones + num_zeros) >= k {
        num_ones
    } else {
        num_ones - cmp::min(num_neg_ones, k - num_ones - num_zeros)
    }
}