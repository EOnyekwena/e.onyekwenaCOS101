//Rust program to read the height of a person
// and then print if person is tall, dwarf,
// or average height person

use std::io;

fn main()
{
    let mut input = String::new();

    println!("\nEnter Your Height (in feet):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 4.11 && height <= 5.6
    {
        println!("You are of Average Height");
    }
    else if height > 5.6 && height <= 6.4
    {
        println!("You are Tall");
    }
    else if height < 4.11 && height > 3.3    
    {
      println!("You are a Dwarf");    
    }
    else
    {
        println!("Abnormal height");

    }
}
