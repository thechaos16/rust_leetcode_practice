use std::collections::HashMap;


fn main() {
    let m = 2;
    let n = 3;
    let indices = vec![vec![0, 1], vec![1, 1]];
    println!("{}", odd_cells(m, n, indices));
}

fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    let mut row_map = HashMap::new();
    let mut col_map = HashMap::new();
    for indice in indices {
        let before_cnt = row_map.entry(indice[0]).or_insert(0);
        *before_cnt += 1;
        let before_cnt = col_map.entry(indice[1]).or_insert(0);
        *before_cnt += 1;
    }
    for row_idx in 0..m {
        let row_res = row_map.entry(row_idx as i32).or_insert(0);
        for col_idx in 0..n {
            let col_res = col_map.entry(col_idx as i32).or_insert(0);
            if (*row_res + *col_res) % 2 == 1 {
                cnt += 1;
            }
        }
    }
    return cnt;
}