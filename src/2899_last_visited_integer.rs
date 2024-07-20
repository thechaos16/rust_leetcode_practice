fn main() {
    let nums = vec![1, 2, -1, -1, -1];
    println!("{:?}", last_visited_integers(nums));
}

fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut seen = vec![];
    let mut consec = 0;
    for num in nums {
        if num != -1 {
            seen.push(num);
            consec = 0;
        } else {
            if consec >= seen.len() {
                ans.push(-1);
            } else {
                ans.push(seen[seen.len() - 1 - consec]);
            }
            consec += 1;
        }
    }
    return ans;
}