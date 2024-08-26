fn main() {
    let num = 28;
    println!("{}", check_perfect_number(num));
}

fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    let mut sum_ = 1;
    for idx in 2..num {
        if (num / idx) < idx {
            break;
        }
        if num % idx == 0 {
            sum_ += idx;
            if idx != num / idx {
                sum_ += num / idx;
            }
        }
    }
    return sum_ == num;
}