fn main(){
    let input = "Hello World".to_string();
    let input2 = "   fly me   to   the moon  ".to_string();
    println!("{}", get_word(input));
    println!("{}", get_word(input2));
}

fn get_word(input: String) -> i32 {
    let mut split = input.split(" ");
    let count = split.clone().count();
    let mut res = 0;
    for idx in 0..count {
        let cur = split.next().unwrap();
        if cur.len() != 0 {
            res = cur.len();
        }
    }
    return res as i32;
}