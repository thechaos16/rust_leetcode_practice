fn main () {
    let input = 20;
    println!("{}", total_money(input));
}

fn total_money(n: i32) -> i32 {
    let mut mutable_n = n;
    let mut idx = 0;
    let mut result = 0;
    while mutable_n > 0 {
        if mutable_n >= 7 {
            result += 28 + idx * 7;
        } else {
            result += (mutable_n * (mutable_n + 1)) / 2 + idx * mutable_n;
        }
        mutable_n -= 7;
        idx += 1;
    }
    return result;
}