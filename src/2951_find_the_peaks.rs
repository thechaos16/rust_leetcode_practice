fn main() {
    let mountains = vec![2, 4, 4];
    println!("{:?}", find_peaks(mountains));
}

fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for idx in 1..(mountain.len() - 1) {
        if mountain[idx] > mountain[idx - 1] && mountain[idx] > mountain[idx + 1] {
            ans.push(idx as i32);
        }
    }
    return ans;
}