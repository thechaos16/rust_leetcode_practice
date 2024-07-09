fn main() {
    let widths = vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];
    let s = "abcdefghijklmnopqrstuvwxyz".to_string();
    println!("{:?}", number_of_lines(widths, s));
}

fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut cur_len = 0;
    let mut cur_line = 1;
    for ch in s.chars() {
        let cur_width = widths[(ch as u8) as usize - 97 as usize];
        if cur_len + cur_width > 100 {
            cur_line += 1;
            cur_len = cur_width;
        } else {
            cur_len += cur_width;
        }
    }
    return vec![cur_line, cur_len];
}