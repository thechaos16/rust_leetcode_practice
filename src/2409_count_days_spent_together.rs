use std::cmp;


fn main() {
    let arrive_alice = "08-15".to_string();
    let leave_alice = "09-18".to_string();
    let arrive_bob = "08-16".to_string();
    let leave_bob = "09-19".to_string();
    println!("{}", count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob));
}

fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
    let late_arrive = cmp::max(arrive_alice, arrive_bob);
    let early_leave = cmp::min(leave_alice, leave_bob);
    if early_leave < late_arrive {
        return 0;
    }
    let dates = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let arrive_vec = late_arrive.split("-").collect::<Vec<_>>();
    let leave_vec = early_leave.split("-").collect::<Vec<_>>();
    if arrive_vec[0] == leave_vec[0] {
        return leave_vec[1].parse::<i32>().unwrap() - arrive_vec[1].parse::<i32>().unwrap() + 1;
    }
    let mut cnt = 0;
    let start = arrive_vec[0].parse::<i32>().unwrap();
    let end = leave_vec[0].parse::<i32>().unwrap();
    cnt += dates[start as usize - 1] - arrive_vec[1].parse::<i32>().unwrap() + 1;
    for idx in (start + 1)..end {
        cnt += dates[idx as usize - 1];
    }
    cnt += leave_vec[1].parse::<i32>().unwrap();
    return cnt;
}