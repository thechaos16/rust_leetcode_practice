fn main() {
    let arr = vec![2, 6, 4, 1];
    println!("{}", three_consecutive_odds(arr));
}

fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut odd_cnt = 0;
    for num in arr.iter() {
        if num % 2 == 0 {
            odd_cnt = 0;
        } else {
            odd_cnt += 1;
            if odd_cnt == 3 {
                return true;
            }
        }
    }
    return false;
}