fn main() {
    let command = "G()(al)".to_string();
    println!("{}", interpret(command));
}

fn interpret(command: String) -> String {
    return command.replace("()", "o").replace("(al)", "al");
}