fn main() {
    let n = 3;
    let k = 5;
    println!("{}", number_of_child(n, k));
}

fn number_of_child(n: i32, k: i32) -> i32 {
    let quot = k / (n - 1);
    let remainder = k % (n - 1);
    if quot % 2 == 0 {
        return remainder;
    } else {
        return n - 1 - remainder;
    }
}