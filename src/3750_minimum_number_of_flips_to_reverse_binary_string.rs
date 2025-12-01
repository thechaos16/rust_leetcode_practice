fn main() {
    let n = 10;
    println!("{}", minimum_flips(n));
}

fn minimum_flips(n: i32) -> i32 {
    let mut binary = vec![];
    let mut mut_n = n;
    while mut_n > 0 {
        let residual = mut_n % 2;
        binary.push(residual);
        mut_n /= 2;
    }
    let mut cnt = 0;
    for idx in 0..(binary.len() / 2) {
        if binary[idx] != binary[binary.len() - idx - 1] {
            cnt += 2;
        }
    }
    return cnt;
}