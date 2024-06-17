use std::collections::HashMap;


fn main() {
    let grid = vec![vec![1, 3], vec![2, 2]];
    println!("{:?}", find_missing_and_repeated_values(grid));
}

fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut hashmap = HashMap::new();
    let mut cnt = 0;
    let mut twice = -1;
    let sum_ans = (1 + grid.len() * grid.len()) * (grid.len() * grid.len()) / 2;
    for row in grid {
        for val in row {
            *hashmap.entry(val).or_insert(0) += 1;
            cnt += val;
            if *hashmap.get(&val).unwrap() == 2 {
                twice = val;
            }
        }
    }
    let mut ans = vec![];
    ans.push(twice);
    ans.push(sum_ans as i32 - cnt + twice);
    return ans
}