use std::io;

fn slices_and_leftovers(people: i32, pizzas: i32, slices: i32) -> (i32, i32) {
    #[allow(unused_assignments)]
    let mut distribution: i32 = 0;
    let mut remainder: i32 = 0;
    if people % (pizzas * slices) == 0 {
        distribution = people / (pizzas * slices);
    } else {
        remainder = people % (pizzas * slices); 
        distribution = (people - remainder) / (pizzas * slices);
    };

    return (distribution, remainder);
}

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();

    println!("How many people?");
    io::stdin().read_line(&mut x).expect("Not an int!");

    println!("How many pizzas do you have?");
    io::stdin().read_line(&mut y).expect("Not an int!");

    println!("How many slices per pizza?");
    io::stdin().read_line(&mut z).expect("Not an int!");

    let people: i32 = x.trim().parse().expect("Not an int!");
    let pizzas: i32 = y.trim().parse().expect("Not an int!");
    let slices: i32 = z.trim().parse().expect("Not an int!");

    let (distribution, leftovers) = slices_and_leftovers(people, pizzas, slices);

    println!("Each person gets {} {} of pizza.",
        distribution, 
        if distribution == 1 { "slice" } else { "slices" });

    if leftovers >= 1 {
        println!("There {} {} leftover {}.",
            if leftovers == 1 { "is" } else { "are" },
            leftovers,
            if leftovers == 1 { "piece" } else { "pieces" });
    }
}
