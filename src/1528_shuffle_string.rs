fn main() {
    let input = "codeleet".to_string();
    let indices = vec![4,5,6,7,0,2,1,3];
    println!("{:?}", restore_string(input, indices));
}

fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut ans = vec![];
    for idx in 0..indices.len() {
        ans.push("".to_string());
    }
    for idx in 0..indices.len() {
        ans[indices[idx] as usize] = s.chars().nth(idx).unwrap().to_string();
    }
    return ans.join("");
}