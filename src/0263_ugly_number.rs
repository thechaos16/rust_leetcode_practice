fn main () {
    let input = 6;
    println!("{}", is_ugly(input));
    let input = 14;
    println!("{}", is_ugly(input));
}

fn is_ugly(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let mut res = n;
    while res % 2 == 0 {
        res /= 2;
    }
    while res % 3 == 0 {
        res /= 3;
    }
    while res % 5 == 0 {
        res /= 5;
    }
    return res == 1;
}