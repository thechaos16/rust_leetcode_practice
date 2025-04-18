use std::collections::HashMap;


fn main () {
    let input = vec![1, 2, 2, 3, 1, 3, 2, 2, 3, 3, 3, 3];
    println!("{}", find_shortest_sub_array(input));
}

fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hash_data: HashMap<i32, (usize, usize, usize)> = HashMap::new();
    let mut max_val = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let before_cnt = hash_data.entry(num).or_insert((0, idx, idx));
        before_cnt.0 += 1;
        before_cnt.2 = idx;
        max_val = max_val.max(before_cnt.0);
    }
    hash_data.values().filter(|&&(freq, _, _)| freq ==max_val).map(|&(_, first, last)| (last - first + 1) as i32).min().unwrap_or(0)
}