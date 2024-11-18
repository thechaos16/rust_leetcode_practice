fn main() {
    let code = vec![2, 4, 9, 3];
    let k = -2;
    println!("{:?}", decrypt(code, k));
}

fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = vec![];
    if k == 0 {
        return vec![0; code.len()];
    } else if k > 0 {
        let mut sum_ = 0;
        for idx in 1..k as usize + 1 {
            sum_ += code[idx];
        }
        for idx in 0..code.len() {
            ans.push(sum_);
            sum_ += code[(idx + k as usize + 1) % code.len()];
            sum_ -= code[(idx + 1) % code.len()];
        }
    } else {
        let mut sum_ = 0;
        for idx in 1..(1 - k) as usize {
            sum_ += code[code.len() - idx];
        }
        for idx in 0..code.len() {
            ans.push(sum_);
            sum_ += code[idx];
            let mut next_idx = idx as i32 + k;
            if next_idx < 0 {
                next_idx = code.len() as i32 + next_idx;
            }
            sum_ -= code[(next_idx % code.len() as i32) as usize];
        }
    }
    return ans;
}