fn main() {
    let input = vec![5, 6, 2, 7, 4];
    println!("{}", max_product_difference(input));
}

fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut max_val = -1;
    let mut max_val_2 = -1;
    let mut min_val = 10001;
    let mut min_val_2 = 10001;
    for num in nums {
        if num >= max_val {
            max_val_2 = max_val;
            max_val = num;
        } else if num >= max_val_2 {
            max_val_2 = num;
        }
        if num <= min_val {
            min_val_2 = min_val;
            min_val = num;
        } else if num <= min_val_2 {
            min_val_2 = num;
        }
    }
    return max_val * max_val_2 - min_val * min_val_2;
}