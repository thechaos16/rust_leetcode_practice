use std::collections::VecDeque;


fn main(){
    let input: &str = "{{}}";
    let input2: &str = "}}";
    let input3 : &str = "[[()]";
    println!("{}", is_valid(&input));
    println!("{}", is_valid(&input2));
    println!("{}", is_valid(&input3));
}

fn is_valid(input_str: &str) -> bool {
    let mut stack: VecDeque<char> = VecDeque::new();

    for one_char in input_str.chars() {
        match one_char {
            '{' | '(' | '[' => stack.push_back(one_char),
            '}' | ')' | ']' => {
                if stack.is_empty() {
                    return false;
                }
                let last_char = stack.pop_back().unwrap();
                if !is_matching_pair(last_char, one_char) {
                    return false;
                }
            },
            _ => return false
        }
    }
    stack.is_empty()
}

fn is_matching_pair(left: char, right: char) -> bool {
    match (left, right) {
        ('{', '}') | ('(', ')') | ('[', ']') => true,
        _ => false
    }
}