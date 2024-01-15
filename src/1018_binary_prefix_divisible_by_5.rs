fn main() {
    let input = vec![0, 1, 1];
    println!("{:?}", prefixes_div_by5(input));
}

fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut res = vec![];
    let mut cur_num = 0;
    for num in nums {
        cur_num *= 2;
        cur_num += num;
        cur_num %= 5;
        res.push(cur_num % 5 == 0);
    }
    return res;
}