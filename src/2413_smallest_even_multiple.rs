fn main () {
    let input = 10;
    println!("{}", smallest_even_multiple(input));
}

fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 {
        return n;
    }
    return 2 * n;
}