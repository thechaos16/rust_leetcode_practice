fn main() {
    let nums1 = vec![1, 3, 4];
    let nums2 = vec![1, 3, 4];
    let k = 1;
    println!("{}", number_of_pairs(nums1, nums2, k));
}

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    for num in nums1.iter() {
        for num2 in nums2.iter() {
            if num % (num2 * k) == 0 {
                count += 1;
            }
        }
    }
    return count;
}