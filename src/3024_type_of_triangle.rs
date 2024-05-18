fn main() {
    let input = vec![3, 3, 3];
    println!("{}", triangle_type(input));
}

fn triangle_type(nums: Vec<i32>) -> String {
    let mut nums_mut = nums.clone();
    nums_mut.sort();
    if nums_mut[2] >= (nums_mut[0] + nums_mut[1]) {
        return "none".to_string();
    }
    if (nums_mut[0] == nums_mut[1]) && (nums_mut[0] == nums_mut[2]) {
        return "equilateral".to_string();
    } else if (nums_mut[0] == nums_mut[1]) || (nums_mut[1] == nums_mut[2]) {
        return "isosceles".to_string();
    } else {
        return "scalene".to_string();
    }
}