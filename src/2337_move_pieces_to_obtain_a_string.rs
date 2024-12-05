fn main() {
    let start = "_L__R__R_".to_string();
    let target = "L______RR".to_string();
    println!("{}", can_change(start, target));
}

fn can_change(start: String, target: String) -> bool {
    let mut st_vec = vec![];
    let mut tg_vec = vec![];
    for (idx, ch) in start.chars().enumerate() {
        if ch != '_' {
            st_vec.push((idx, ch));
        }
    }
    for (idx, ch) in target.chars().enumerate() {
        if ch != '_' {
            tg_vec.push((idx, ch));
        }
    }
    if st_vec.len() != tg_vec.len() {
        return false;
    }
    for idx in 0..st_vec.len() {
        if st_vec[idx].1 != tg_vec[idx].1 {
            return false;
        }
        if st_vec[idx].1 == 'L' && st_vec[idx].0 < tg_vec[idx].0 {
            return false;
        }
        if st_vec[idx].1 == 'R' && st_vec[idx].0 > tg_vec[idx].0 {
            return false;
        }
    }
    return true;
}