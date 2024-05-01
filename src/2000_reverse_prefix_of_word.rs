fn main() {
    let input = "abcdefd".to_string();
    let ch = "d".to_string();
    println!("{}", reverse_prefix(input, ch.chars().nth(0).unwrap()));
}

fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(split_idx) = word.find(ch) {
        let reversed_prefix: String = word[..=split_idx].chars().rev().collect();
        return reversed_prefix + &word[split_idx + 1..];
    }
    word
}