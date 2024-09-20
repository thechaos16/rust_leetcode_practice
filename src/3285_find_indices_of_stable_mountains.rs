fn main() {
    let height = vec![1, 2, 3, 4, 5];
    let threshold = 2;
    println!("{:?}", stable_mountains(height, threshold));
}

fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut ans = vec![];
    for idx in 0..(height.len() - 1) {
        if height[idx] > threshold {
            ans.push((idx + 1) as i32);
        }
    }
    return ans;
}