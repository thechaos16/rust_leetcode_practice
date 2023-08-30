use std::cmp;


fn main () {
    let input = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![1, 2]];
    println!("{}", num_equiv_domino_pairs(input));
}

fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    for idx in 0..dominoes.len() {
        let min_val = cmp::min(dominoes[idx][0], dominoes[idx][1]);
        let max_val = cmp::max(dominoes[idx][0], dominoes[idx][1]);
        let key1 = min_val * 10 + max_val;
        for idx2 in idx + 1..dominoes.len() {
            let min_val = cmp::min(dominoes[idx2][0], dominoes[idx2][1]);
            let max_val = cmp::max(dominoes[idx2][0], dominoes[idx2][1]);
            let key2 = min_val * 10 + max_val;
            if key1 == key2 {
                cnt += 1;
            }
        }
    }
    return cnt;
}