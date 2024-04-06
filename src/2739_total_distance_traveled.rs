fn main() {
    let main_tank = 5;
    let additional_tank = 10;
    println!("{}", distance_traveled(main_tank, additional_tank));
}

fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
    let mut cnt = 0;
    let mut cur_tank = main_tank;
    let mut cur_add = additional_tank;
    while cur_tank >= 5 {
        cur_tank -= 5;
        cnt += 5;
        if cur_add >= 1 {
            cur_add -= 1;
            cur_tank += 1;
        }
    }
    cnt += cur_tank;
    return cnt * 10;
}