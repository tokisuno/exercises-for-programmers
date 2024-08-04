use std::io;

fn squared_ft() -> (f32, f32) {
    let mut input = String::new();
    println!("What is the length of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer!");
    let x: f32 = input.trim().parse().expect("Not an integer");

    let mut input = String::new();
    println!("What is the width of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer");
    let y: f32 = input.trim().parse().expect("Not an integer!");

    (x, y)
}

fn squared_inches() -> (f32, f32) {
    let mut input = String::new();
    println!("What is the length of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer!");
    let x: f32 = input.trim().parse().expect("Not an integer");

    let mut input = String::new();
    println!("What is the width of the room in feet?");
    io::stdin().read_line(&mut input).expect("Not an integer");
    let y: f32 = input.trim().parse().expect("Not an integer!");
    
    (x, y)
}

fn main() -> io::Result<()> {
    let mut i = String::new();
    println!("[f]eet or [i]nches? (press enter when chosen");
    io::stdin().read_line(&mut i)?;

    if i.trim() == "f" || i.trim() == "F" {
        let (length, width) = squared_inches();
        println!("{}, {}", length, width);
    }
    if i.trim() == "i" || i.trim() == "I" {
        let (length, width) = squared_ft();
        println!("{}, {}", length, width);
    }

    Ok(())
}
