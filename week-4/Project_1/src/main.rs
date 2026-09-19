//Rust Program to determine the qudratic roots of an equation

use std::io;

fn main()
{


let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

println!("Enter your first constant");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

println!("Enter your second constant");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");

println!("Enter your third constant");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let c:f32 = input3.trim().parse().expect("Not a valid number");
let d:f32 = (b * b) - 4.0 * (a * c);

if  d > 0.0 { 
    let x1: f32 = (- b + d.sqrt()) / (2.0 * a);
    let x2: f32 = (- b - d.sqrt()) / (2.0 * a);

   println!("There are two distinct roots: {}",d);
   println!("The first root is: {}", x1);
   println!("The second root is {}", x2 );
}

    else if d == 0.0 {
        let _x: f32 = - b / (2.0 * a);

        println!("There is one repeated root: {}",_x);
    }

    else {

        println!("This has no real roots: {}",d);
    }

}