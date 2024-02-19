fn main() {
    let original = vec![1, 2, 3, 4];
    let m = 2;
    let n = 2;
    println!("{:?}", construct2_d_array(original, m, n));
}

fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if original.len() as i32 != m * n {
        return vec![];
    }
    let mut res = vec![];
    for row_idx in 0..m {
        res.push(original[((row_idx * n) as usize)..(((row_idx + 1) * n) as usize)].to_vec());
    }
    return res;
}