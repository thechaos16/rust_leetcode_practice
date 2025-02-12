fn main() {
    let s = "45320".to_string();
    println!("{}", get_smallest_string(s));
}

fn get_smallest_string(s: String) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    for idx in 0..(chars.len() - 1) {
        if chars[idx] > chars[idx + 1] {
            if idx == 0 && chars[idx + 1] == '0' {
                continue;
            } else {
                let temp = chars[idx + 1];
                chars[idx + 1] = chars[idx];
                chars[idx] = temp;
                break;
            }
        }
    }
    return chars.iter().collect();
}