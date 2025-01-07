fn main() {
    let n = 8;
    println!("{}", smallest_number(n));
}

fn smallest_number(n: i32) -> i32 {
    let num = 2_i32.pow((n as f32).log2() as u32 + 1) - 1;
    return num;
}