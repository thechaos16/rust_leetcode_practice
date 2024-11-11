fn main() {
    let k = 5;
    println!("{}", kth_character(k));
}

fn kth_character(k: i32) -> char {
    //let a_int = 'a'.to_digit(10 as u32).unwrap();
    let mut start_str = vec!['a'];
    while start_str.len() < k as usize {
        for idx in 0..start_str.len() {
            if start_str[idx] == 'z' {
                start_str.push('a');
            } else {
                start_str.push(char::from_u32(start_str[idx] as u32 + 1).unwrap());
            }
        }
    }
    return start_str[k as usize - 1];
}