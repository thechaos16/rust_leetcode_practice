fn main() {
    let mat = vec![vec![0, 1], vec![1, 0]];
    println!("{:?}", row_and_maximum_ones(mat));
}

fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans_idx = 0;
    let mut min_cnt = 0;
    for row_idx in 0..mat.len() {
        let mut cur_cnt = 0;
        for col in mat[row_idx].iter() {
            cur_cnt += col;
        }
        if cur_cnt > min_cnt {
            ans_idx = row_idx as i32;
            min_cnt = cur_cnt;
        }
    }
    return vec![ans_idx, min_cnt];
}