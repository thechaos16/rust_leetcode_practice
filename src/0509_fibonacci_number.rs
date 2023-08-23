
fn main () {
    let input = 5;
    println!("{}", fib(input));
}

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2)
}