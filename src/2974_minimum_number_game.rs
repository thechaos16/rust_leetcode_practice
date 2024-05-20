fn main() {
    let input = vec![5, 4, 2, 3];
    println!("{:?}", number_game(input));
}

fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut num_mut = nums.clone();
    num_mut.sort();
    let mut ans = vec![];
    for idx in 0..num_mut.len() {
        if idx % 2 == 0 {
            ans.push(num_mut[idx + 1]);
        } else {
            ans.push(num_mut[idx - 1]);
        }
    }
    return ans;
}