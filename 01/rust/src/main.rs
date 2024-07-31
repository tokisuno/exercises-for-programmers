use std::io;

fn main() {
    println!("What is your name?: ");
    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).expect("failed to readline");

    println!("Hello, {}, nice to meet you!", input_string.trim());
}
