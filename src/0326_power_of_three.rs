fn main() {
    let input = 1298;
    println!("{}", is_power_of_three(input));
}

fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    } else if n == 1 {
        return true;
    } else if n % 3 != 0 {
        return false;
    } else {
        return is_power_of_three(n / 3);
    }
}