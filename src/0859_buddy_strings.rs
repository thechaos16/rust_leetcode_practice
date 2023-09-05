fn main() {
    let input1 = "abc".to_string();
    let input2 = "bad".to_string();

    println!("{}", buddy_strings(input1, input2));
}

fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let mut diff1 = vec![];
    let mut diff2 = vec![];
    for idx in 0..s.len() {
        let s_char = s.chars().nth(idx).unwrap();
        let g_char = goal.chars().nth(idx).unwrap();
        if s_char != g_char {
            diff1.push(s_char);
            diff2.push(g_char);
        }
    }
    if diff1.len() != 2 {
        return false;
    }
    if diff1[0] == diff2[1] && diff1[1] == diff2[0] {
        return true;
    } else {
        return false;
    }
}