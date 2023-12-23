fn main() {
    let input1 = vec![1, 3];
    let input2 = vec![];
    println!("{}", find_median_sorted_arrays(input1, input2));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() + nums2.len() == 1 {
        if nums1.len() == 0 {
            return nums2[0] as f64;
        } else {
            return nums1[0] as f64;
        }
    }
    let mut median_vec = vec![0, 0];
    let target = (nums1.len() + nums2.len()) / 2;
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut cur = 0;
    while idx1 + idx2 <= target {
        if idx2 >= nums2.len() {
            cur = nums1[idx1];
            idx1 += 1;
        } else if idx1 >= nums1.len() {
            cur = nums2[idx2];
            idx2 += 1;
        } else if nums1[idx1] <= nums2[idx2] {
            cur = nums1[idx1];
            idx1 += 1;
        } else {
            cur = nums2[idx2];
            idx2 += 1;
        }
        median_vec[0] = median_vec[1];
        median_vec[1] = cur;
    }
    if (nums1.len() + nums2.len()) % 2 == 1 {
        return median_vec[1] as f64;
    } else {
        return ((median_vec[0] + median_vec[1]) as f64) / 2.0;
    }
}