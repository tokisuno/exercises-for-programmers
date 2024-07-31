use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    loop {
        println!("What is the input string? ");
        io::stdin().read_line(&mut input)?;
        let _input = input.trim();
        if _input.len() < 1 {
            println!("String is too short, try again!");
            continue;
        } else {
            break;
        }
    } 

    println!("{} has {} characters.", input.trim(), input.trim().len());

    Ok(())
}
