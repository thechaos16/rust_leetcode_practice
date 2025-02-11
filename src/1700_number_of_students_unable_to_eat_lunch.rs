fn main() {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    println!("{}", count_students(students, sandwiches));
}

fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut student_counts = [0; 2];
    for &preference in students.iter() {
        student_counts[preference as usize] += 1;
    }
    for &sandwich in sandwiches.iter() {
        if student_counts[sandwich as usize] > 0 {
            student_counts[sandwich as usize] -= 1;
        } else {
            return student_counts[0] + student_counts[1];
        }
    }
    return 0;
}