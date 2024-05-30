fn main() {
    let prices = vec![90,29,6,74];
    let money = 82;
    println!("{}", buy_choco(prices, money));
}

fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut min1 = -1;
    let mut min2 = -1;
    for price in prices {
        if min1 == -1 {
            min1 = price;
        } else if min1 > price {
            min2 = min1;
            min1 = price;
        } else if min2 == -1 {
            min2 = price;
        }  else if min2 > price {
            min2 = price;
        }
    }
    if min1 + min2 > money {
        return money;
    } else {
        return money - (min1 + min2);
    }
}