fn main () {
    let s = "dart".to_string();
    let k = 3;
    println!("{}", get_encrypted_string(s, k));
}

fn get_encrypted_string(s: String, k: i32) -> String {
    let mut ans = "".to_string();
    for idx in 0..s.len() {
        let new_num = (idx as i32 + k) % s.len() as i32;
        ans.push(s.chars().nth(new_num as usize).unwrap());
    }
    return ans;
}