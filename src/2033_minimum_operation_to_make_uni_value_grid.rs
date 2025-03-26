fn main() {
    let grid = vec![vec![2, 4], vec![6, 8]];
    let x = 2;
    println!("{}", min_operations(grid, x));
}

fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut flat_vec: Vec<_> = grid.into_iter().flatten().collect();
    flat_vec.sort();
    let median = flat_vec[flat_vec.len() / 2 as usize];
    let mut diff = 0;
    for elem in flat_vec {
        let cur_diff = (elem - median).abs();
        if cur_diff % x != 0 {
            return -1;
        } else {
            diff += cur_diff / x;
        }
    }
    return diff;
}