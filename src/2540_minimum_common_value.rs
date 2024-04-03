fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![4, 5, 6];
    println!("{}", get_common(nums1, nums2));
}

fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut idx1 = 0;
    let mut idx2 = 0;
    loop {
        if idx1 >= nums1.len() || idx2 >= nums2.len() {
            return -1;
        }
        let num1 = nums1[idx1];
        let num2 = nums2[idx2];
        if num1 == num2 {
            return num1;
        } else if num1 < num2 {
            idx1 += 1;
        } else {
            idx2 += 1;
        }
    }
}