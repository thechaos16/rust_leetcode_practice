fn main() {
    let n = 10;
    println!("{}", arrange_coins(n));
}

fn arrange_coins(n: i32) -> i32 {
    let mut cnt = 1;
    let mut n_mut = n;
    while n_mut > 0 {
        if n_mut < cnt {
            return cnt - 1;
        }
        n_mut -= cnt;
        cnt += 1;
    }
    return cnt - 1;
}