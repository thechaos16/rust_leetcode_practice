use std::cmp::Ordering;


fn main () {
    let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights = vec![180, 165, 170];
    println!("{:?}", sort_people(names, heights));
}

fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut integrate_vec = vec![];
    for idx in 0..names.len() {
        integrate_vec.push([names[idx].clone(), heights[idx].to_string()]);
    }
    integrate_vec.sort_by(|a, b| {
        if a[1].parse::<i32>().unwrap() > b[1].parse::<i32>().unwrap() {
            Ordering::Less
        } else if a[1].parse::<i32>().unwrap() < b[1].parse::<i32>().unwrap() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut result = vec![];
    for idx in 0..integrate_vec.len() {
        result.push(integrate_vec[idx].clone()[0].clone());
    }
    return result;
}