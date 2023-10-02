use std::cmp;


fn main() {
    let input = "AAABBAABBBBA".to_string();
    println!("{}", winner_of_game(input));
}

fn winner_of_game(colors:String) -> bool {
    let (mut a_cnt, mut b_cnt, mut last_a, mut last_b) = (0, 0, 0, 0);
    for ch in colors.chars() {
        if ch == 'A' {
            last_a += 1;
            b_cnt += cmp::max(0, last_b - 2);
            last_b = 0;
        } else {
            last_b += 1;
            a_cnt += cmp::max(0, last_a - 2);
            last_a = 0;
        }
    }
    a_cnt += cmp::max(0, last_a - 2);
    b_cnt += cmp::max(0, last_b - 2);
    return a_cnt > b_cnt;
}