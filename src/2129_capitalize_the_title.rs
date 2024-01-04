fn main() {
    let input = "capiTalIze tHe titLe".to_string();
    println!("{}", capitalize_title(input));
}

fn capitalize_title(title: String) -> String {
    let words = title.split(" ").collect::<Vec<_>>();
    let mut new_string = "".to_string();
    for (word_idx, &word) in words.iter().enumerate() {
        for idx in 0..word.len() {
            let ch = word.chars().nth(idx).unwrap();
            if idx == 0 && word.len() > 2 {
                new_string.push_str(&ch.to_uppercase().to_string());
            } else {
                new_string.push_str(&ch.to_lowercase().to_string());
            }
        }
        if word_idx == words.len() - 1 {
            break;
        }
        new_string.push_str(&" ".to_string());
    }
    return new_string;
}