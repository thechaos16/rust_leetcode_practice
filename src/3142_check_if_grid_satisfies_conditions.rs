fn main() {
    let grid = vec![vec![1, 0, 2], vec![1, 0, 2]];
    println!("{}", satisfies_conditions(grid));
}

fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if row < grid.len() - 1 && grid[row][col] != grid[row + 1][col]{
                return false;
            }
            if  col < grid[row].len() - 1 && grid[row][col] == grid[row][col + 1] {
                return false;
            }
        }
    }
    return true;
}