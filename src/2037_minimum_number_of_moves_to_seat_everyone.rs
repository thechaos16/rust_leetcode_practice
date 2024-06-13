fn main() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    println!("{}", min_moves_to_seat(seats, students));
}

fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seat_mut = seats;
    let mut stu_mut = students;
    seat_mut.sort();
    stu_mut.sort();
    let mut cnt = 0;
    for idx in 0..seat_mut.len() {
        cnt += (seat_mut[idx] - stu_mut[idx]).abs();
    }
    return cnt;
}