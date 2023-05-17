fn main() {
    let input = vec![1, 2, 3];
    println!("{:?}", plus_one(input));
    let input = vec![9, 9, 9];
    println!("{:?}", plus_one(input));
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut last = 1;
    for idx in (0..digits.len()).rev() {
        let sum_ = last + digits[idx];
        result.push(sum_ % 10);
        last = sum_ / 10;
    }
    if last == 1 {
        result.push(1);
    }
    result.reverse();
    return result;
}