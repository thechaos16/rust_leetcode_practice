fn main(){
    let input = 5;
    println!("{:?}", sum_zero(input));
    let input = 4;
    println!("{:?}", sum_zero(input));
}

fn sum_zero(n: i32) -> Vec<i32> {
    let mut result = vec![];
    if n % 2 == 1 {
        result.append(&mut vec![0]);
    }
    for idx in 1..(n/2 + 1) {
        result.append(&mut vec![idx, -idx])
    }
    return result;
}