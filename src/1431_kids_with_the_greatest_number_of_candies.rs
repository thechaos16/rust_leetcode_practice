
fn main(){
    let candies = vec![2,3,5,1,3];
    let extra_candies = 3;
    let candies2 = vec![4,2,1,1,2];
    let extra_candies2 = 1;
    println!("{:?}", check_candies(candies, extra_candies));
    println!("{:?}", check_candies(candies2, extra_candies2));
}

fn check_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_val = candies.iter().max().unwrap();
    let mut res = vec![];
    for idx in 0..candies.len() {
        if candies[idx] + extra_candies >= *max_val {
            res.append(&mut vec![true]);
        } else {
            res.append(&mut vec![false]);
        }
    }
    return res;
}