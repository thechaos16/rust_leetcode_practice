fn main (){
    let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{}", diagonal_sum(input));
}

pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for idx in 0..mat.len() {
        ans += mat[idx][idx];
        if mat.len() - idx - 1 == idx {
            continue;
        }
        ans += mat[idx][mat.len() - idx - 1];
    }
    return ans;
}