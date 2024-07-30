fn main() {
    let matrix = vec![
        vec!["B".to_string(), "W".to_string(), "B".to_string()],
        vec!["B".to_string(), "W".to_string(), "W".to_string()],
        vec!["B".to_string(), "W".to_string(), "B".to_string()]
    ];
    println!("{}", can_make_square(matrix));
}

fn can_make_square(grid: Vec<Vec<String>>) -> bool {
    for row_idx in 0..2 {
        for col_idx in 0..2 {
            let mut b_cnt = 0;
            if grid[row_idx][col_idx] == "B".to_string() {
                b_cnt += 1;
            }
            if grid[row_idx + 1][col_idx] == "B".to_string() {
                b_cnt += 1;
            }
            if grid[row_idx][col_idx + 1] == "B".to_string() {
                b_cnt += 1;
            }
            if grid[row_idx + 1][col_idx + 1] == "B".to_string() {
                b_cnt += 1;
            }
            if b_cnt != 2 {
                return true;
            }
        }
    }
    return false;
}