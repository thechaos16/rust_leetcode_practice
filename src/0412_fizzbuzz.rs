fn main () {
    let input = 15;
    println!("{:?}", fizz_buzz(input));
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for idx in 1..n + 1 {
        if idx % 3 == 0 && idx % 5 == 0 {
            res.push("FizzBuzz".to_string());
        } else if idx % 3 == 0 {
            res.push("Fizz".to_string());
        } else if idx % 5 == 0 {
            res.push("Buzz".to_string());
        } else {
            res.push(idx.to_string());
        }
    }
    return res;
}