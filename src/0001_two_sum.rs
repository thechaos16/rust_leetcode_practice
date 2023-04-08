
fn main() {
    let nums1 = [2,7,11,15];
    let target1: i32 = 9;
    let nums2 = [3,2,4];
    let target2: i32 = 6;

    println!("{:?}", two_sum((&nums1).to_vec(), target1));
    println!("{:?}", two_sum((&nums2).to_vec(), target2));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for idx in 0..nums.len() {
        for idx2 in (idx + 1)..nums.len() {
            if nums[idx] + nums[idx2] == target {
                return [idx as i32, idx2 as i32].to_vec();
            }
        }
    }
    return [].to_vec();
}

