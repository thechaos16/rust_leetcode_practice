fn main(){
    let input = vec!["-5".to_string(),"2".to_string(),"C".to_string(),"D".to_string(),"+".to_string()];
    //let input = vec!["1".to_string() ,"C".to_string()];
    println!("{}", cal_points(input));
}

fn cal_points(operations: Vec<String>) -> i32 {
    let mut result = vec![];
    for ops in operations {
        match ops.chars().nth(0).unwrap() {
            'C' => {
                let _ = result.pop().unwrap();
            },
            'D' => {
                let last = result[result.len() - 1];
                result.push(last * 2);
            },
            '+' => {
                let last = result[result.len()  - 1];
                let last2 = result[result.len() - 2];
                result.push(last + last2);
            },
            _ => {
                result.push(ops.trim().parse().unwrap());
            }
        }
    }
    return result.iter().sum();
}