fn main() {
    let nums1 = vec![3, 2, 0, 1, 0];
    let nums2 = vec![6, 5, 0];
    println!("{}", min_sum(nums1, nums2));
}

fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let (mut one_sum, mut one_cnt, mut two_sum, mut two_cnt) = (0 as i64, 0 as i64, 0 as i64, 0 as i64);
    for num in nums1 {
        if num == 0 {
            one_sum += 1;
            one_cnt += 1;
        } else {
            one_sum += num as i64;
        }
    }
    for num in nums2 {
        if num == 0 {
            two_sum += 1;
            two_cnt += 1;
        } else {
            two_sum += num as i64;
        }
    }
    if one_sum == two_sum {
        return one_sum;
    } else if one_sum > two_sum {
        if two_cnt == 0 {
            return -1;
        } else {
            return one_sum;
        }
    } else {
        if one_cnt == 0 {
            return -1;
        } else {
            return two_sum;
        }
    }
}