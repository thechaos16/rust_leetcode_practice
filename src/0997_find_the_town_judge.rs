use std::collections::HashMap;


fn main() {
    let n = 2;
    let input = vec![vec![1, 2]];
    println!("{}", find_judge(n, input));
}

fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut trust_cnt = HashMap::new();
    let mut trusted_cnt = HashMap::new();
    for tr in trust {
        let before_cnt = trust_cnt.entry(tr[0]).or_insert(0);
        *before_cnt += 1;
        let before_cnt = trusted_cnt.entry(tr[1]).or_insert(0);
        *before_cnt += 1;
    }
    for idx in 1..(n + 1) {
        let tr = trust_cnt.entry(idx).or_insert(0);
        let tred = trusted_cnt.entry(idx).or_insert(0);
        if *tr == 0 && *tred == n - 1 {
            return idx;
        }
    }
    return -1;
}