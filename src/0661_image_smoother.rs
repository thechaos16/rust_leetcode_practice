fn main() {
    let input = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    println!("{:?}", image_smoother(input));
}

fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut answer_vec = [].to_vec();
    let row_len = img.len();
    let col_len = img[0].len();
    for row_idx in 0..row_len {
        let mut one_row = [].to_vec();
        for col_idx in 0..col_len {
            let mut cur_val = img[row_idx][col_idx];
            let mut cnt = 1;
            if row_idx > 0 {
                if col_idx > 0 {
                    cur_val += img[row_idx - 1][col_idx - 1];
                    cnt += 1;
                }
                cur_val += img[row_idx - 1][col_idx];
                cnt += 1;
                if col_idx < (col_len - 1) {
                    cur_val += img[row_idx - 1][col_idx + 1];
                    cnt += 1;
                }
            }
            
            if col_idx > 0 {
                cur_val += img[row_idx][col_idx - 1];
                cnt += 1;
            }
            if col_idx < (col_len - 1) {
                cur_val += img[row_idx][col_idx + 1];
                cnt += 1;
            }

            if row_idx < (row_len - 1) {
                if col_idx > 0 {
                    cur_val += img[row_idx + 1][col_idx - 1];
                    cnt += 1;
                }
                cur_val += img[row_idx + 1][col_idx];
                cnt += 1;
                if col_idx < (col_len - 1) {
                    cur_val += img[row_idx + 1][col_idx + 1];
                    cnt += 1;
                }
            }
            one_row.push((cur_val / cnt) as i32);
        }
        answer_vec.push(one_row);
    }
    return answer_vec;
}