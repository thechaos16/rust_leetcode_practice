fn main() {
    let input = "HellO".to_string();
    println!("{}", to_lower_case(input));
}

fn to_lower_case(s: String) -> String {
    return s.to_lowercase();
}