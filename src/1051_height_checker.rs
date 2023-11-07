fn main() {
    let input = vec![1, 1, 4, 2, 1, 3];
    println!("{}", height_checker(input));
}

fn height_checker(heights: Vec<i32>) -> i32 {
    let mut copy_vec = heights.clone();
    copy_vec.sort();
    let mut cnt = 0;
    for idx in 0..copy_vec.len() {
        if heights[idx] != copy_vec[idx] {
            cnt += 1;
        }
    }
    return cnt;
}