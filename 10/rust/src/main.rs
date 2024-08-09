use std::io;
use std::collections::HashMap;

fn main() {
    let mut items: HashMap<&str, f32> = HashMap::new();

    let mut i1 = String::new();
    let mut i2 = String::new();
    let mut i3 = String::new();

    let mut q1 = String::new();
    let mut q2 = String::new();
    let mut q3 = String::new();

    let mut total: f32 = 0.0;

    println!("Enter the price of item 1");
    io::stdin().read_line(&mut i1).expect("Not an int!");
    println!("Enter the quantity of item 1");
    io::stdin().read_line(&mut q1).expect("Not an int!");
    let (item1_price, item1_quantity): (f32, f32) = (
            i1.trim().parse::<f32>().expect("Not an int!"),
            q1.trim().parse::<f32>().expect("Not an int!"),
        );
    items.insert("i1", item1_price * item1_quantity);

    println!("Enter the price of item 2");
    io::stdin().read_line(&mut i2).expect("Not an int!");
    println!("Enter the quantity of item 2");
    io::stdin().read_line(&mut q2).expect("Not an int!");
    let (item2_price, item2_quantity): (f32, f32) = (
            i2.trim().parse::<f32>().expect("Not an int!"),
            q2.trim().parse::<f32>().expect("Not an int!"),
        );
    items.insert("i2", item2_price * item2_quantity);


    println!("Enter the price of item 3");
    io::stdin().read_line(&mut i3).expect("Not an int!");
    println!("Enter the quantity of item 3");
    io::stdin().read_line(&mut q3).expect("Not an int!");
    let (item3_price, item3_quantity): (f32, f32) = (
            i3.trim().parse::<f32>().expect("Not an int!"),
            q3.trim().parse::<f32>().expect("Not an int!"),
        );

    items.insert("i3", item3_price * item3_quantity);

    for value in items.values() {
        total += value; 
    }
    let tax = total * 0.055;
    println!("Subtotal: ${total:.2}");
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", total + tax);
}
