fn main() {
    let boxes = "110".to_string();
    println!("{:?}", min_operations(boxes));
}

fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    let chars: Vec<char> = boxes.chars().collect();
    let mut result = vec![0; n];
    let (mut left_one, mut right_one, mut total_ops) = (0, 0, 0);
    for (i, &c) in chars.iter().enumerate() {
        if c == '1' {
            right_one += 1;
            total_ops += i;
        }
    }
    for i in 0..n {
        if i > 0 {
            if chars[i - 1] == '1' {
                left_one += 1;
                right_one -= 1;
            }
            total_ops += left_one - right_one;
        }
        result[i] = total_ops as i32;
    }
    result
}
