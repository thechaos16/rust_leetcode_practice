
fn main() {
    let input1 = ["flower","flow","flight"];
    let input2 = ["dog","racecar","car"];
    println!("{}", run(&input1));
    println!("{}", run(&input2));
}

fn run<'a>(arr: &'a [&'a str]) -> &'a str {
    let mut count: usize = 0;
    let mut min_size: u32 = 99999;
    for i1 in arr.iter() {
        if min_size > i1.len() as u32{
            min_size = i1.len() as u32;
        }
    }
    if min_size == 0 {
        ""
    } else {
        let mut first_str: char;
        let mut okay: bool = true;
        loop {
            first_str = arr[0].chars().nth(count).unwrap();
            for i1 in arr.iter() {
                if first_str != i1.chars().nth(count).unwrap() {
                    okay = false;
                    break;
                }
            }
            if !okay {
                break;
            }
            count += 1;
            if count == min_size as usize {
                break;
            }
        }
        &arr[0][..count]
    }
}