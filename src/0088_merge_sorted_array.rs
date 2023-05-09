fn main() {
    let arr1 = &mut vec![7, 8, 9, 0, 0, 0] as &mut Vec<i32>; 
    let arr2 = &mut vec![2, 5, 6] as &mut Vec<i32>;
    let m = 3;
    let n = 3;
    merge(arr1, m, arr2, n);
    println!("{:?}", &arr1);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx1 = m - 1;
    let mut idx2 = n - 1;
    let mut real_idx = nums1.len() - 1;
    while idx1 >= 0 && idx2 >= 0 {
        if nums1[idx1 as usize] < nums2[idx2 as usize] {
            nums1[real_idx] = nums2[idx2 as usize];
            idx2 -= 1;
        } else {
            nums1[real_idx] = nums1[idx1 as usize];
            nums1[idx1 as usize] = 0;
            idx1 -= 1;
        }
        real_idx -= 1;
    }
    while idx2 >= 0 {
        nums1[real_idx] = nums2[idx2 as usize];
        idx2 -= 1;
        if real_idx == 0 {
            break;
        }
        real_idx -= 1;
    }
}