fn main() {
    let n = 2;
    let commands = vec!["RIGHT".to_string(), "DOWN".to_string()];
    println!("{}", final_position_of_snake(n, commands));
}

fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    let mut start_x = 0;
    let mut start_y = 0;
    for command in commands {
        if command == "RIGHT".to_string() {
            if start_y < n - 1 {
                start_y += 1;
            }
        } else if command == "LEFT".to_string() {
            if start_y > 0 {
                start_y -= 1;
            }
        } else if command == "UP".to_string() {
            if start_x > 0 {
                start_x -= 1;
            }
        } else {
            if start_x < n - 1 { 
                start_x += 1;
            }
        }
    }
    return start_x * n + start_y;
}
