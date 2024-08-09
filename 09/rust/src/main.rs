use std::io;

const GALLON_IN_SQFT: i32 = 350;

fn conversion(length: i32, width: i32) -> i32 {
    let square_ft: i32 = length * width; 
    if square_ft % GALLON_IN_SQFT == 0 {
        return square_ft / GALLON_IN_SQFT;
    } else {
        return (square_ft / GALLON_IN_SQFT) + 1;
    }
}

fn main() {
    let mut length = String::new();
    let mut width = String::new();

    println!("Enter length");
    io::stdin().read_line(&mut length).expect("Not an int!");
    let l: i32 = length.trim().parse().expect("Not an int!");

    println!("Enter width");
    io::stdin().read_line(&mut width).expect("Not an int!");
    let w: i32 = width.trim().parse().expect("Not an int!");

    let gallons = conversion(l, w);

    println!("You will need to purchase {} {} of paint to cover {} square feet", gallons, if gallons == 1 { "gallon" } else { "gallons" }, l*w);
}
