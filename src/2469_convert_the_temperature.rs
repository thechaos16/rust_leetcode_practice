fn main() {
    let celsius = 36.50;
    println!("{:?}", convert_temperature(celsius));
}

fn convert_temperature(celsius: f64) -> Vec<f64> {
    return vec![celsius + 273.15 as f64, celsius * 1.80 + 32.0];
}