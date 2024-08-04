use std::io;

fn main() {
    let mut input = String::new();
    println!("What is the length of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer!");
    let x: f32 = input.trim().parse().expect("Not an integer");

    let mut input = String::new();
    println!("What is the width of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer");
    let y: f32 = input.trim().parse().expect("Not an integer!");

    println!("The area is");
    println!("{} square feet", x*y);
    println!("{:.3} square meters", (x*y) * 0.09290304);
}
