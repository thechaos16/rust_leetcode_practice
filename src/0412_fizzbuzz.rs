fn main () {
    let input = 15;
    println!("{:?}", fizz_buzz(input));
}

fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n).map(|num| match (num % 3 == 0, num % 5 == 0){
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        _ => num.to_string(),
    }).collect()
}