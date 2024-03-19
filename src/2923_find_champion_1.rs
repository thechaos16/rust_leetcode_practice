fn main() {
    let input = vec![vec![0, 1], vec![0, 0]];
    println!("{}", find_champion(input));
}

fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    for idx in 0..grid.len() {
        let mut sum_ = 0;
        for idx2 in 0..grid[idx].len() {
            sum_ += grid[idx][idx2];
        }
        if sum_ == grid.len() as i32 - 1 {
            return idx as i32;
        }
    }
    return -1;
}