use std::collections::HashMap;


fn main() {
    let input = 45;
    println!("{}", climb_stairs(input));
}

fn climb_stairs(n: i32) -> i32 {
    return run(n, &mut HashMap::new());
}

fn run(n: i32, hashmap: &mut HashMap<i32, i32>) -> i32 {
    match hashmap.get(&n).map(|entry| entry.clone()) {
        Some(result) => result,
        None => {
            let result = match n {
                1 => 1,
                2 => 2,
                a => run(a - 1, hashmap) + run(a - 2, hashmap),
            };
            hashmap.insert(n, result.clone());
            result
        }
    }
}