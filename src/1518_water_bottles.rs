fn main() {
    let num_bottles = 9;
    let num_exchange = 3;
    println!("{}", num_water_bottles(num_bottles, num_exchange));
}

fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let quotient = num_bottles / num_exchange;
    let remainder = num_bottles % num_exchange;
    if quotient == 0 {
        return num_bottles;
    } else {
        return quotient * num_exchange + num_water_bottles(quotient + remainder, num_exchange);
    }
}