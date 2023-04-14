fn main(){
    let input = [1, 3, 5, 6, 9];
    let target = 8;
    println!("{}", search((&input).to_vec(), target));
}

fn search<'a>(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() as i32 == 0{
        return 0;
    }
    let pivot: usize = nums.len() / 2;
    let mut res = nums.len() as i32;
    if nums[pivot] == target {
        res = pivot as i32;
    } else if nums[pivot] > target {
        res = search(nums[..pivot].to_vec(), target);
    } else {
        res = search(nums[(pivot + 1)..].to_vec(), target) + pivot as i32 + 1;
    }
    return res;
}
