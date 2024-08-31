fn main() {
    let n = 16;
    println!("{}", is_power_of_two(n));
}

fn is_power_of_two(n: i32) -> bool {
    if n == 1 {
        return true;
    }
    if n <= 0 {
        return false;
    }
    let mut mutable_n = n;
    while mutable_n > 1 {
        if mutable_n % 2 != 0 {
            return false;
        }
        mutable_n /= 2;
    }
    return true;
}