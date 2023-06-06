fn main() {
    let input = vec![3, 5, 1];
    println!("{}", can_make_arithmatic_progression(input));
}

fn can_make_arithmatic_progression(arr: Vec<i32>) -> bool {
    let mut arr2 = vec![];
    for val in arr{
        arr2.push(val);
    }
    arr2.sort();
    let diff = arr2[1] - arr2[0];
    for i in 1..(arr2.len() - 1) {
        if arr2[i + 1] - arr2[i] != diff {
            return false;
        }
    }
    return true;
}