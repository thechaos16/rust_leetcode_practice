use std::cmp::Ordering;


fn main() {
    let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
    let truck_size = 4;
    println!("{}", maximum_units(box_types, truck_size));
}

fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut boxes = box_types.clone();
    boxes.sort_by(|a, b| {
        if a[1] >= b[1] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut ans = 0;
    let mut size = truck_size;
    for idx in 0..boxes.len() {
        if boxes[idx][0] >= size {
            ans += boxes[idx][1] * size;
            break;
        } else {
            ans += boxes[idx][1] * boxes[idx][0];
            size -= boxes[idx][0];
        }
    }
    return ans;
}