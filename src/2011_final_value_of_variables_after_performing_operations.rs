fn main() {
    let operations = vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];
    println!("{}", final_value_after_operations(operations));
}

fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut ans = 0;
    for string in operations.iter() {
        if string.chars().nth(0).unwrap() == '-' || string.chars().nth(string.len() - 1).unwrap() == '-' {
            ans -= 1;
        } else {
            ans += 1;
        }
    }
    return ans;
}