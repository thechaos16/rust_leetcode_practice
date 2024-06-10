fn main() {
    let x = 18;
    println!("{}", sum_of_the_digits_of_harshad_number(x));
}

fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut sum_ = 0;
    let mut x_mut = x;
    while x_mut > 0 {
        sum_ += x_mut % 10;
        x_mut /= 10;
    }
    if x % sum_ == 0 {
        return sum_;
    }
    return -1;
}