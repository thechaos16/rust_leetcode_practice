fn main() {
    let boxes = "110".to_string();
    println!("{:?}", min_operations(boxes));
}

fn min_operations(boxes: String) -> Vec<i32> {
    let (mut left_cnt, mut right_cnt, mut cur_ans) = (0, 0, 0);
    let mut ans = vec![];
    for idx in 0..boxes.len() {
        let box_ = boxes.chars().nth(idx).unwrap();
        if box_ == '1' {
            if idx != 0 {
                right_cnt += 1;
            }
            cur_ans += idx;
        }
    }
    for idx in 0..boxes.len() {
        if idx != 0 {
            if boxes.chars().nth(idx - 1).unwrap() == '1' {
                left_cnt += 1;
            } 
            cur_ans += left_cnt - right_cnt;
            if boxes.chars().nth(idx).unwrap() == '1' {
                right_cnt -= 1;
            }
        }
        ans.push(cur_ans as i32);
    }
    return ans;
}