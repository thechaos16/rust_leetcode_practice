use std::collections::LinkedList;

fn main(){
    let input: &str = "{{}}";
    let input2: &str = "}}";
    let input3 : &str = "[[()]";
    println!("{}", is_valid(&input));
    println!("{}", is_valid(&input2));
    println!("{}", is_valid(&input3));
}

fn is_valid(input_str: &str) -> bool {
    let mut stack: LinkedList<char> = Default::default();
    let mut result: bool = true;
    for idx in 0..input_str.len() {
        let one_char = input_str.chars().nth(idx);
        if one_char == Some('{') || one_char == Some('[') || one_char == Some('(') {
            stack.push_back(one_char.unwrap());
        } else {
            if stack.len() == 0 {
                result = false;
                break
            }
            let last_char: char = stack.pop_back().unwrap();
            if (one_char == Some('}') && last_char == '{') || (one_char == Some(']') && last_char == '[') || (one_char == Some(')') && last_char == '(') {
                continue;
            } else {
                result = false;
                break
            }
        }
    }
    if stack.len() > 0 {
        return false;
    } else {
        return result;
    }
}