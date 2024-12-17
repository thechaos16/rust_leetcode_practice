fn main() {
    let n = 4;
    let time = 5;
    println!("{}", pass_the_pillow(n, time));
}

fn pass_the_pillow(n: i32, time: i32) -> i32{
    let divider = time / (n - 1);
    let remainder = time % (n - 1);
    if divider % 2 == 0 {
        return remainder + 1;
    } else {
        return n - remainder;
    }
}