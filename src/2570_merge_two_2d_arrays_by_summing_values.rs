fn main() {
    let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
    let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
    println!("{:?}", merge_arrays(nums1, nums2));
}

fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let (mut idx1, mut idx2) = (0, 0);
    while idx1 < nums1.len() && idx2 < nums2.len() {
        if nums1[idx1][0] < nums2[idx2][0] {
            ans.push(nums1[idx1].clone());
            idx1 += 1;
        } else if nums1[idx1][0] == nums2[idx2][0] {
            ans.push(vec![nums1[idx1][0], nums1[idx1][1] + nums2[idx2][1]]);
            idx1 += 1;
            idx2 += 1;
        } else {
            ans.push(nums2[idx2].clone());
            idx2 += 1;
        }
    }
    if idx1 < nums1.len() {
        for idx in idx1..nums1.len() {
            ans.push(nums1[idx].clone());
        }
    }
    if idx2 < nums2.len() {
        for idx in idx2..nums2.len() {
            ans.push(nums2[idx].clone());
        }
    }
    return ans;
}