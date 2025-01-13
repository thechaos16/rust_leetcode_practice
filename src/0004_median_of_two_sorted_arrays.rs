use std::cmp::Ordering;

fn main() {
    let input1 = vec![1, 3];
    let input2 = vec![];
    println!("{}", find_median_sorted_arrays(input1, input2));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_len = nums1.len() + nums2.len();
    let target = total_len / 2;
    let (mut current, mut prev, mut idx1, mut idx2) = (0, 0, 0, 0);
    while idx1 + idx2 <= target {
        prev = current;
        match (nums1.get(idx1), nums2.get(idx2)) {
            (Some(&val1), Some(&val2)) => {
                match val1.cmp(&val2) {
                    Ordering::Less | Ordering::Equal => {
                        current = val1;
                        idx1 += 1;
                    }
                    Ordering::Greater => {
                        current = val2;
                        idx2 += 1;
                    }
                }
            }
            (Some(&val1), None) => {
                current = val1;
                idx1 += 1;
            }
            (None, Some(&val2)) => {
                current = val2;
                idx2 += 1;
            }
            (None, None) => break,
        }
    }
    if total_len % 2 == 1 {
        current as f64
    } else {
        (current as f64 + prev as f64) / 2.0
    }
}