use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main () {
    let mut input: Vec<i32> = vec![4,2,4,0,0,3,0,5,1,0];
    move_zeros(&mut input);
    println!("{:?}", input);
}

fn move_zeros(nums: &mut Vec<i32>) {
    let mut deq = BinaryHeap::new();
    for idx in 0..nums.len() {
        if nums[idx] == 0 {
            deq.push(Reverse(idx));
        }
    }
    let mut skip_cnt = 0;
    if deq.len() != 0{
        for idx in 0..nums.len() {
            if nums[idx] == 0 {
                skip_cnt += 1;
                continue;
            } else if skip_cnt == 0 {
                continue;
            }
            let zero_idx = deq.pop().unwrap().0;
            nums[zero_idx] = nums[idx];
            nums[idx] = 0;
            deq.push(Reverse(idx));
        }
    }
}