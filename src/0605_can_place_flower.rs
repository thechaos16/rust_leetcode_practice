fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    println!("{}", can_place_flowers(flowerbed, n));
}

fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut ans = 0;
    let mut adj_zero_cnt = 0;
    let mut left_close = 0;
    for flower in flowerbed {
        if flower == 1 {
            ans += (adj_zero_cnt - left_close) / 2;
            adj_zero_cnt = 0;
            left_close = 1;
        } else {
            adj_zero_cnt += 1;
        }
    }
    ans += (adj_zero_cnt - left_close + 1) / 2;
    return ans >= n;
}