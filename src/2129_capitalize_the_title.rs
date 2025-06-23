fn main() {
    let input = "capiTalIze tHe titLe".to_string();
    println!("{}", capitalize_title(input));
}

fn capitalize_title(title: String) -> String {
    title.split_whitespace().map(|word| {
        if word.len() > 2 {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        } else {
            word.to_lowercase()
        }
    }).collect::<Vec<_>>().join(" ")
}
