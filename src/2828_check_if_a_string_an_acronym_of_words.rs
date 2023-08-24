fn main () {
    let input = vec!["alice".to_string(),"bob".to_string(),"charlie".to_string()];
    let s = "ab".to_string();
    println!("{}", is_acronym(input, s));
}

fn is_acronym(words: Vec<String>, s: String) -> bool {
    let mut answer: String = "".to_string();
    for word in words {
        answer.push_str(&word.chars().nth(0).unwrap().to_string());
    }
    return answer == s;
}