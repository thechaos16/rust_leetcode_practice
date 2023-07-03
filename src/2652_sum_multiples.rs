fn main() {
    let input = 10;
    println!("{}", sum_of_multiples(input));
}

fn sum_of_multiples(n: i32) -> i32 {
    let mut answer = 0;
    for idx in 1..(n + 1) {
        if idx % 3 == 0 || idx % 5 == 0 || idx % 7 == 0 {
            answer += idx;
        }
    }
    return answer
}