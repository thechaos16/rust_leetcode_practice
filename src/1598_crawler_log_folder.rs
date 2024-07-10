fn main() {
    let input = vec!["d1/".to_string(), "d2/".to_string(), "../".to_string(), "d21/".to_string(), "./".to_string()];
    println!("{}", min_operations(input));
}

fn min_operations(logs: Vec<String>) -> i32 {
    let mut level = 0;
    for log in logs.iter() {
        if *log == "./".to_string() {
            continue;
        } else if *log == "../".to_string() {
            level -= 1;
            if level == -1 {
                level = 0;
            }
        } else {
            level += 1;
        }
    }
    return level;
}