fn main() {
    let input = vec![vec![1,1,0,0,0], vec![1,1,1,1,0], vec![1,0,0,0,0], vec![1,1,0,0,0], vec![1,1,1,1,1]];
    let k = 3;
    println!("{:?}", k_weakest_rows(input, k));
}

fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut result = vec![];
    for idx in 0..mat.len() {
        let sum_ = mat[idx].iter().sum();
        result.push(vec![sum_, idx as i32]);
    }
    result.sort();
    let mut ans = vec![];
    for idx in 0..k {
        ans.push(result[idx as usize][1]);
    }
    return ans;
}