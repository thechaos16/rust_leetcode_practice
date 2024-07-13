use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let nums1 = vec![4, 1, 3];
    let nums2 = vec![5, 7];
    println!("{}", min_number(nums1, nums2));
}

fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let hashset: HashSet<i32> = HashSet::from_iter(nums1.clone());
    let mut common = 10;
    let mut num1_min = 10;
    let mut num2_min = 10;
    for num in nums1 {
        if num1_min > num {
            num1_min = num;
        }
    }
    for num in nums2 {
        if hashset.contains(&num) {
            if common > num {
                common = num;
            }
        }
        if num2_min > num {
            num2_min = num;
        }
    }
    if common != 10 {
        return common;
    } else {
        if num1_min > num2_min {
            return num2_min * 10 + num1_min;
        } else {
            return num1_min * 10 + num2_min;
        }
    }
}