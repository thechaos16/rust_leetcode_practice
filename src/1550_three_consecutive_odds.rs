fn main() {
    let arr = vec![2, 6, 4, 1];
    println!("{}", three_consecutive_odds(arr));
}

fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.iter().map(|num| num % 2 == 1).collect::<Vec<bool>>().windows(3).any(|window| window.iter().all(|&is_odd| is_odd))
}