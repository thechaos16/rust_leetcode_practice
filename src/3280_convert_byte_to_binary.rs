fn main() {
    let date = "2080-02-29".to_string();
    println!("{}", convert_date_to_binary(date));
}

fn convert_date_to_binary(date: String) -> String {
    let parts = date.split("-");
    let mut ans = vec![];
    for part in parts {
        let cur_int: i32 = part.parse().unwrap();
        ans.push(format!("{:b}", cur_int));
    }
    return ans.join("-");
}