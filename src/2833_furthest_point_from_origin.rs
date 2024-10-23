fn main() {
    let moves = "L_RL__R".to_string();
    println!("{}", furthest_distance_from_origin(moves));
}

fn furthest_distance_from_origin(moves: String) -> i32 {
    let (mut left, mut right, mut neutral) = (0, 0, 0);
    for ch in moves.chars() {
        if ch == 'R' {
            right += 1;
        } else if ch == 'L' {
            left += 1;
        } else {
            neutral += 1;
        }
    }
    return (left - right as i32).abs() + neutral;
}