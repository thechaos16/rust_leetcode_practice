fn main() {
    let input = vec![0, 1, 2, 3, 4];
    let target = 2;
    println!("{}", number_of_employees_who_met_target(input, target));
}

fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    let mut cnt = 0;
    for hour in &hours {
        if *hour >= target {
            cnt += 1;
        }
    }
    return cnt;
}