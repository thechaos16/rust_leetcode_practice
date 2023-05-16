fn main () {
    let input = "leEeetcode";
    println!("{}", make_good((&input).to_string()));
    let input = "abBAcC";
    println!("{}", make_good((&input).to_string()));
}

fn make_good(s: String) -> String {
    let mut stack = vec![];
    for idx in 0..s.len() {
        let character = s.chars().nth(idx).unwrap();
        if stack.len() == 0 {
            stack.push(character);
        } else {
            if stack[stack.len() - 1] != character && stack[stack.len() - 1].to_lowercase().to_string() == character.to_lowercase().to_string() {
                stack.pop();
            } else {
                stack.push(character);
            }
        }
    }
    return stack.into_iter().collect();
}