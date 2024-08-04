use std::io;
use chrono::Datelike;
use std::process;


fn retirement_time(age: i32, ideal: i32) -> (i32, i32) {
    let current_year = chrono::Utc::now().year();
    let years_left: i32 = ideal - age;
    let final_year: i32 = current_year + years_left;

    return (years_left, final_year);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("What is your current age?");
    io::stdin().read_line(&mut input)?;
    let current_age: i32 = input.trim().parse().expect("Not an integer");

    let mut input = String::new();
    println!("At what age would you like to retire?");
    io::stdin().read_line(&mut input)?;
    let ideal_age: i32 = input.trim().parse().expect("Not an integer");

    let (current_year, final_year) = retirement_time(current_age, ideal_age);

    if ideal_age < 0 {
        eprintln!("Jeez, just retire already...");
        process::exit(0);
    }
    println!("You have {} years left until you retire, which would be the year: {}!", current_year, final_year);

    Ok(())
}
