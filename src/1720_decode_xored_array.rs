
fn main() {
    let input = vec![6,2,7,3];
    let first = 4;
    println!("{:?}", decode(input, first));
}

fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut answer = vec![];
    let mut cur = first;
    answer.push(first);
    for encode in encoded {
        cur ^= encode;
        answer.push(cur);
    }
    return answer;
}