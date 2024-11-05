fn main() {
    let nums = vec![1, 2, 3];
    println!("{}", maximum_products(nums));
}

fn maximum_products(nums: Vec<i32>) -> i32 {
    if nums.len() == 3 {
        return nums[0] * nums[1] * nums[2];
    }
    let mut plus = vec![];
    let mut minus = vec![];
    let mut zero = 0;
    for num in nums {
        if num > 0 {
            plus.push(num);
        } else if num < 0 {
            minus.push(num);
        } else {
            zero += 1;
        }
    }
    plus.sort();
    minus.sort();
    if minus.len() == 1 {
        if plus.len() >= 3 {
            return plus[plus.len() - 1] * plus[plus.len() - 2] * plus[plus.len() - 3];
        } else {
            if zero >= 1 {
                return 0;
            } else {
                return plus[0] * plus[1] * minus[0];
            }
        }
    } else if plus.len() == 0 {
        if zero >= 1 {
            return 0;
        } else {
            return minus[minus.len() - 1] * minus[minus.len() - 2] * minus[minus.len() - 3];
        }
    } else if minus.len() == 0 {
        if plus.len() >= 3 {
            return plus[plus.len() - 1] * plus[plus.len() - 2] * plus[plus.len() - 3];
        } else {
            return 0;
        }
    } else if plus.len() < 3 {
        if minus.len() >= 2 {
            return plus[plus.len() - 1] * minus[0] * minus[1];
        } else {
            return 0;
        }
    } else {
        let all_plus = plus[plus.len() - 1] * plus[plus.len() - 2] * plus[plus.len() - 3];
        let one_plus = plus[plus.len() - 1] * minus[0] * minus[1];
        if all_plus >= one_plus {
            return all_plus;
        } else {
            return one_plus;
        }
    }
}