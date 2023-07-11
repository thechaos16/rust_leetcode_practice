fn main () {
    let arr = vec![3, 0, 1, 1, 9, 7];
    let a = 7;
    let b = 2;
    let c = 3;
    println!("{}", count_good_triples(arr, a, b, c));
}

fn count_good_triples(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;
    for idx in 0..arr.len() {
        for idx2 in idx + 1..arr.len() {
            for idx3 in idx2 + 1..arr.len() {
                if (arr[idx] - arr[idx2]).abs() <= a && (arr[idx2] - arr[idx3]).abs() <= b && (arr[idx3] - arr[idx]).abs() <= c {
                    count += 1;
                }
            }
        }
    }
    return count;
}