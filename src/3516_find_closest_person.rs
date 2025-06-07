fn main() {
    let x = 2;
    let y = 7;
    let z = 4;
    println!("{}", find_closets(x, y, z));
}

fn find_closets(x: i32, y: i32, z: i32) -> i32 {
    let diff = (z - x).abs() - (z - y).abs();
    if diff == 0 {
        return 0;
    } else if diff > 0 {
        return 2;
    } else {
        return 1;
    }
}