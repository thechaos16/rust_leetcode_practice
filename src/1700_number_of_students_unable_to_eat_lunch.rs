fn main() {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    println!("{}", count_students(students, sandwiches));
}

fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut diff = 0;
    for idx in 0..students.len() {
        diff += students[idx];
        diff -= sandwiches[idx];
    }
    if diff == 0 {
        return 0;
    } else if diff > 0 {
        let mut zero_cnt = 0;
        let mut ans = 0;
        for idx in (0..sandwiches.len()).rev() {
            if sandwiches[idx] == 0 {
                zero_cnt += 1;
            }
            if zero_cnt == diff {
                ans = (sandwiches.len() - idx) as i32;
                break
            }
        }
        return ans;
    } else {
        let mut one_cnt = 0;
        let mut ans = 0;
        for idx in (0..sandwiches.len()).rev() {
            if sandwiches[idx] == 1 {
                one_cnt += 1;
            }
            if one_cnt == -diff {
                ans = (sandwiches.len() - idx) as i32;
                break
            }
        }
        return ans;
    }
}