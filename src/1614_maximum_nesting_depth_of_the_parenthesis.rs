fn main() {
    let input = "(1+(2*3)+((8)/4))+1".to_string();
    println!("{}", max_depth(input));
}

fn max_depth(s: String) -> i32 {
    let mut stack_cnt = 0;
    let mut max_val = 0;
    for ch in s.chars() {
        if ch == '(' {
            stack_cnt += 1;
            if stack_cnt > max_val {
                max_val = stack_cnt;
            }
        } else if ch == ')' {
            stack_cnt -= 1;
        }
    }
    return max_val;
}