fn main() {
    let input = vec![3,8,-10,23,19,-4,-14,27];
    println!("{:?}", minimum_abs_difference(input));
}

fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vector = arr;
    vector.sort();
    let mut ans = vec![];
    let mut min_diff = 0;
    for idx in 0..vector.len() - 1 {
        if min_diff == 0 || min_diff > (vector[idx + 1] - vector[idx]).abs() {
            min_diff = (vector[idx + 1] - vector[idx]).abs();
        }
    }
    for idx in 0..vector.len() - 1 {
        if (vector[idx + 1] - vector[idx]).abs() == min_diff {
            ans.push(vec![vector[idx], vector[idx + 1]]);
        }
    }
    return ans;
}