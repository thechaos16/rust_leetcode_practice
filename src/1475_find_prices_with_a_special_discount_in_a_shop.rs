fn main() {
    let prices = vec![8, 4, 6, 2, 3];
    println!("{:?}", final_prices(prices));
}

fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut ans = prices.clone();
    for idx in 0..ans.len() {
        for idx2 in (idx + 1)..ans.len() {
            if ans[idx2] <= ans[idx] {
                ans[idx] = ans[idx] - ans[idx2];
                break;
            }
        }
    }
    return ans;
}