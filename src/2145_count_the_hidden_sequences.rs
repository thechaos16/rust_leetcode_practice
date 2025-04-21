fn main() {
    let differences = vec![1, -3, 4];
    let lower = 1;
    let upper = 6;
    println!("{}", number_of_arrays(differences, lower, upper));
}

fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let (mut start, mut min_val, mut max_val) = (0, 0, 0);
    for diff in differences.iter() {
        start += diff;
        min_val = min_val.min(start);
        max_val = max_val.max(start);
    }
    let given = upper as i64 - lower as i64;
    let new = max_val as i64 - min_val as i64;
    if given < new {
        return 0;
    }
    (given - new + 1) as i32
}