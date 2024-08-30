use std::collections::HashMap;


fn main() {
    let n = 4;
    let pick = vec![vec![0,0], vec![1,0], vec![1,0], vec![2,1], vec![2,1], vec![2,0]];
    println!("{}", winning_player_count(n, pick));
}

fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    let mut ans = vec![];
    for _ in 0..n {
        ans.push(HashMap::new());
    }
    for one_pick in pick {
        *ans[one_pick[0] as usize].entry(one_pick[1]).or_insert(0) += 1;
    }
    for idx in 0..n {
        let mut max_val = 0;
        let target: Vec<_> = ans[idx as usize].iter().collect();
        for (_, count) in target {
            if *count >= max_val {
                max_val = *count;
            }
        }
        if max_val >= idx + 1 {
            cnt += 1;
        }
    }
    return cnt;
}