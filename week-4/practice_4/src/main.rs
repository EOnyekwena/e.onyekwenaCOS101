//Rust program to determine age pass

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name please: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your age please: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("Welcome to the party {}!", input2);
    } else {
        println!("Oops, you are a little underage for this party {}", input2);
    }
}

