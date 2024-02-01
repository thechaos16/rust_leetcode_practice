
fn main( ){
    let name = "alex".to_string();
    let typed = "aaleexa".to_string();
    println!("{}", is_long_pressed_name(name, typed));
}

fn is_long_pressed_name(name: String, typed: String) -> bool {
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut name_cnt = 0;
    let mut typed_cnt = 0;
    while idx1 < name.len() {
        let cur_char = name.chars().nth(idx1);
        if typed.chars().nth(idx2) != cur_char {
            return false;
        }
        while idx1 < name.len() && name.chars().nth(idx1 + 1) == cur_char {
            idx1 += 1;
            name_cnt += 1;
        }
        while idx2 < typed.len() && typed.chars().nth(idx2 + 1) == cur_char {
            idx2 += 1;
            typed_cnt += 1;
        }
        if name_cnt > typed_cnt {
            return false;
        }
        idx1 += 1;
        idx2 += 1;
        name_cnt = 0;
        typed_cnt = 0;
    }
    if idx2 != typed.len() {
        return false;
    }
    return true;
}