fn main () {
    let input = vec!['c', 'f', 'j'];
    let target = 'd';
    println!("{}", next_greater_letter(input, target));
}

fn next_greater_letter(letters: Vec<char>, target: char) -> char {
    let mut res: char = '{';
    let mut real_min: char = 'z';
    for c in letters {
        if c > target && res > c{
            res = c;
        }
        if real_min > c {
            real_min = c;
        } 
    }
    if res == '{' {
        return real_min;
    } else {
        return res;
    }
}