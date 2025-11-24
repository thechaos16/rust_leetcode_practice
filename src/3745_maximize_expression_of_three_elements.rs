fn main() {
    let nums = vec![1, 4, 2, 5];
    println!("{}", maximize_expression_of_three(nums));
}

fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
    let (mut max0, mut max1, mut min0) = (-101, -101, 101);
    for num in nums {
        if num > max0 {
            max1 = max0;
            max0 = num;
        } else if num > max1 {
            max1 = num;
        }
        if num < min0 {
            min0 = num;
        }
    }
    return max0 + max1 - min0;
}