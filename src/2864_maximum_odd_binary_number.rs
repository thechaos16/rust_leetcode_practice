fn main() {
    let s = "010".to_string();
    println!("{}", maximum_odd_binary_number(s));
}

fn maximum_odd_binary_number(s: String) -> String {
    let mut one_cnt = 0;
    for ch in s.chars() {
        if ch == '1' {
            one_cnt += 1;
        }
    }
    let mut ans = "".to_string();
    for idx in 0..(one_cnt - 1) {
        ans.push('1');
    }
    for idx in 0..(s.len() - one_cnt) {
        ans.push('0');
    }
    ans.push('1');
    return ans;
}