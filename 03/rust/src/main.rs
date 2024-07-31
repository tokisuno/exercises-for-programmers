use std::io;

fn main() -> io::Result<()>{
    let mut quote = String::new();
    let mut author = String::new();

    println!("What is the quote?");
    io::stdin().read_line(&mut quote)?;

    println!("Who said it?");
    io::stdin().read_line(&mut author)?;

    // I don't think you can do traditional string concatenation in rust...
    println!("{} says, \"{}\"", author.trim(), quote.trim());

    Ok(())
}
