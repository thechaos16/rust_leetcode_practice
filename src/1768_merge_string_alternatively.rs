fn main() {
    let str1 = "abcccccc".to_string();
    let str2 = "pqr".to_string();
    println!("{}", merge_alternatively(str1, str2));
}

fn merge_alternatively(word1: String, word2: String) -> String {
    let mut short = word1.clone();
    let mut long = word2.clone();
    if word1.len() >= word2.len() {
        short = word2.clone();
        long = word1.clone();
    } 
    let mut res = String::from("");
    for idx in 0..short.len() {
        res.push_str(&word1.chars().nth(idx).unwrap().to_string());
        res.push_str(&word2.chars().nth(idx).unwrap().to_string());
    }
    res.push_str(&long[short.len()..]);
    return res;
}