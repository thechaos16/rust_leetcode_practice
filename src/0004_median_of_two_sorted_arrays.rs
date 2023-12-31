fn main() {
    let input1 = vec![1, 3];
    let input2 = vec![];
    println!("{}", find_median_sorted_arrays(input1, input2));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut median_vec = vec![];
    let total_len = nums1.len() + nums2.len();
    let target = total_len/ 2;
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < nums1.len() && idx2 < nums2.len() {
        if nums1[idx1] <= nums2[idx2] {
            median_vec.push(nums1[idx1]);
            idx1 += 1;
        } else {
            median_vec.push(nums2[idx2]);
            idx2 += 1;
        }
    }
    median_vec.extend_from_slice(&nums1[idx1..]);
    median_vec.extend_from_slice(&nums2[idx2..]);
    if total_len % 2 == 1 {
        return median_vec[target] as f64;
    } else {
        return (median_vec[target - 1] + median_vec[target]) as f64 / 2.0;
    }
}