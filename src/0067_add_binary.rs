fn main () {
    let input1 = "11".to_string();
    let input2 = "1".to_string();
    println!("{}", add_binary(input1, input2));
}

fn add_binary(a: String, b: String) -> String {
    if a == "0".to_string() && b == "0".to_string() {
        return "0".to_string();
    }
    let a_int = convert_str_to_int(a);
    let b_int = convert_str_to_int(b);
    return convert_int_to_str(a_int + b_int);
}

fn convert_str_to_int(a: String) -> i32 {
    let mut val = 0;
    for ch in a.chars() {
        let cur_int = ch.to_digit(10 as u32).unwrap();
        val *= 2;
        val += cur_int;
    }
    return val as i32;
}

fn convert_int_to_str(a: i32) -> String {
    let mut a_ = a;
    let mut vector = vec![];
    while a_ != 0 {
        vector.push(char::from_digit((a_ % 2) as u32, 2).unwrap());
        a_ /= 2;
    }
    return vector.into_iter().rev().collect();
}