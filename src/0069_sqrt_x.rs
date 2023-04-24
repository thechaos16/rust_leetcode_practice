fn main() {
    let input = 2147483647;
    println!("{}", get_sqrt(input));
}

fn get_sqrt(x: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    let mut res = 1;
    for idx in 1..x {
        if idx * idx == x {
            res = idx;
            break;
        } else if idx * idx > x{
            res = idx - 1;
            break
        }
    }
    return res;
}