fn main() {
    let start = 10;
    let goal = 7;
    println!("{}", min_bit_flips(start, goal));
}

fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut st = start;
    let mut go = goal;
    let mut cnt = 0;
    loop {
        if st == 0 && go == 0 {
            break;
        }
        if st % 2 != go % 2{
            cnt += 1;
        }
        st /= 2;
        go /= 2;
    }
    return cnt;
}