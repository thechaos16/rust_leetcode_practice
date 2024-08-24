fn main() {
    let nums = vec![3, 2, 1];
    println!("{}", third_max(nums));
}

fn third_max(nums: Vec<i32>) -> i32 {
    let mut three = vec![];
    for num in nums {
        if three.contains(&num) {
            continue;
        }
        three.push(num);
        three.sort();
        three.reverse();
        if three.len() > 3 {
            _ = three.pop();
        }
    }
    if three.len() < 3 {
        return three[0];
    } 
    return three[2];
}