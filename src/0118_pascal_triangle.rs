fn main(){
    let input = 5;
    let input2 = 11;
    println!("{:?}", run_pascal(input));
    println!("{:?}", run_pascal(input2));
}

fn run_pascal(input: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![1]];
    for idx in 1..input {
        let mut new_vec = vec![1];
        for idx2 in 0..res[idx as usize - 1].len() - 1 {
            new_vec.append(&mut vec![res[idx as usize - 1][idx2 as usize] + res[idx as usize - 1][idx2 as usize +1]]);
        }
        new_vec.append(&mut vec![1]);
        res.append(&mut vec![new_vec]);
    }
    return res;
}