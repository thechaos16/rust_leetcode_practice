fn main() {
    let n = 886996;
    println!("{}", alternate_digit_sum(n));
}

fn alternate_digit_sum(n: i32) -> i32 {
    let mut idx = 0;
    let mut mut_n = n;
    let mut res = 0;
    while mut_n > 0 {
        if idx % 2 == 0 {
            res += mut_n % 10;
        } else {
            res -= mut_n % 10;
        }
        mut_n /= 10;
        idx += 1;
    }
    if idx % 2 == 1 {
        return res;
    } else {
        return -res;
    }
}