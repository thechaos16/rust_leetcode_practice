fn main () {
    let input = vec![-5,1,5,0,-7];
    println!("{}", largest_altitude(input));
}

fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut cur_gain = 0;
    let mut max_gain = 0;
    for num in gain {
        cur_gain += num;
        if cur_gain >= max_gain {
            max_gain = cur_gain;
        }
    }
    return max_gain;
}