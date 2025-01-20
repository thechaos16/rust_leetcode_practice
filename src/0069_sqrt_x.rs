fn main() {
    let input = 2147483647;
    println!("{}", get_sqrt(input));
}

fn get_sqrt(x: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    for idx in 1..x {
        if idx * idx == x {
            return idx as i64;
        } else if idx * idx > x{
            return (idx - 1) as i64;
        }
    }
    return 1 as i64;
}