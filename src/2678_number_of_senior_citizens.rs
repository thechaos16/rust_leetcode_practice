fn main() {
    let input = vec!["7868190130M7522".to_string(), "5303914400F9211".to_string(), "9273338290F4010".to_string()];
    println!("{}", count_senors(input));
}

fn count_senors(details: Vec<String>) -> i32 {
    let mut cnt = 0;
    for detail in details {
        if detail.chars().nth(11).unwrap().to_digit(10 as u32) > Some(6) {
            cnt += 1;
        } else if detail.chars().nth(11).unwrap().to_digit(10 as u32) == Some(6) && detail.chars().nth(12).unwrap().to_digit(10 as u32) != Some(0) {
            cnt += 1;
        }
    }
    return cnt;
}