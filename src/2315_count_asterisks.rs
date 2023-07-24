fn main () {
    let input = "l|*e*et|c**o|*de|".to_string();
    println!("{}", counter_asterisks(input));
}

fn counter_asterisks(s: String) -> i32 {
    let mut count = 0;
    let mut opened_bar = 0;
    for ch in s.chars() {
        if ch.to_string() == "|".to_string() {
            opened_bar += 1;
            opened_bar %= 2;
            continue;
        }
        if ch.to_string() == "*".to_string() && opened_bar == 0 {
            count += 1;
        }
    }
    return count;
}