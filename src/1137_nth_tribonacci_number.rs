fn main() {
    let n = 25;
    println!("{}", tribonacci(n));
}

fn tribonacci(n: i32) -> i32 {
    if n < 3 { return (n > 0) as i32; }
    (2..n).fold((1, 1, 0), |(a, b, c), _| (a + b + c, a, b)).0
}