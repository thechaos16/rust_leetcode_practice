fn main() {
    let bills = vec![5, 5, 5, 5, 10, 20];
    println!("{}", lemonade_change(bills));
}

fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut available_bills = [0, 0];
    for bill in bills {
        match bill {
            5 => {
                available_bills[0] += 1;
            },
            10 => {
                if available_bills[0] == 0 {
                    return false;
                }
                available_bills[0] -= 1;
                available_bills[1] += 1;
            },
            20 => {
                if available_bills[0] >= 1 && available_bills[1] >= 1 {
                    available_bills[0] -= 1;
                    available_bills[1] -= 1;
                } else if available_bills[0] >= 3 {
                    available_bills[0] -= 3;
                } else {
                    return false;
                }
            },
            _ => panic!("Invalid bill demoniation {}", bill),
        }
    }
    true
}