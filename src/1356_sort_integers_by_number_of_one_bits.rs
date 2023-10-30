use std::cmp::Ordering;


fn main() {
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", sortByBits(input));
}

fn sortByBits(arr: Vec<i32>) -> Vec<i32> {
    let mut mut_arr = arr.clone();
    mut_arr.sort_by(|a, b|
        if get_bits(*b) > get_bits(*a) {
            Ordering::Less
        } else if get_bits(*b) < get_bits(*a) {
            Ordering::Greater
        } else {
            if *b >= *a {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    );
    return mut_arr;
}

fn get_bits(num: i32) -> i32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return num % 2 + get_bits(num / 2);
    }
}