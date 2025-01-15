fn main() {
    let battery_percentages = vec![1, 1, 2, 1, 3];
    println!("{}", count_tested_devices(battery_percentages));
}

fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    let (mut ans, mut cur_cnt) = (0, 0);
    for num in battery_percentages {
        if num > cur_cnt {
            ans += 1;
            cur_cnt += 1;
        }
    }
    return ans;
}