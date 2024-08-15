use std::cmp::min;


fn main() {
    let bills = vec![5, 5, 5, 5, 10, 20];
    println!("{}", lemonade_change(bills));
}

fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut cur_change = vec![0, 0, 0];
    for bill in bills {
        if bill == 5 {
            cur_change[2] += 1;
        } else if bill == 10 {
            cur_change[1] += 1;
        } else {
            cur_change[0] += 1;
        }
        let mut change = bill - 5;
        if change == 0 {
            continue;
        }
        let twenty = min(cur_change[0], change / 20);
        cur_change[0] -= twenty;
        change -= twenty * 20;
        let ten = min(cur_change[1], change / 10);
        cur_change[1] -= ten;
        change -= ten * 10;
        let five = min(cur_change[2], change / 5);
        cur_change[2] -= five;
        change -= five * 5;
        if change > 0 {
            return false;
        }
    }
    return true;
}