use std::collections::HashMap;


fn main(){
    let nums = vec![1, 2, 3, 2];
    println!("{}", sum_of_unique(nums));
}

fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for num in nums.iter() {
        *count.entry(num).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (key, value) in count {
        if value == 1 {
            ans += key;
        }
    }
    return ans;
}