use std::io;
use std::process;

fn get_int(which: &str) -> f32 {
    let mut input = String::new();
    println!("Enter {} number", which);
    io::stdin().read_line(&mut input).expect("no input found");
    let input = input.trim();
    let value: f32 = input.trim().parse().expect("input is not numeric");
    if value < 0.0 {
        eprintln!("input can't be negative");
        process::exit(0);
    }
    return value;
}

fn main() -> io::Result<()>{
    let x = get_int("first");
    let y = get_int("second");

    println!("{} {} {} = {}", x, "+", y, x+y);
    println!("{} {} {} = {}", x, "-", y, x-y);
    println!("{} {} {} = {}", x, "*", y, x*y);
    println!("{} {} {} = {}", x, "/", y, x/y);

    Ok(())
}
