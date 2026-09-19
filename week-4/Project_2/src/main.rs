//The Incentive Calculator on Employee's Experience and Age

use std::io;

fn main(){
println!("The incentive calculator on Employee's age and experience");
    let mut age = String::new();

    println!("Enter your age please: ");
    io::stdin().read_line(&mut age).unwrap();
    let age:u32 = age.trim().parse().unwrap();
  let mut experience = String::new();

    println!("Are you an experienced individaul? (yes/no) ");
    io::stdin().read_line(&mut experience).unwrap();

    let experience = experience.trim().to_lowercase();
    

let incentive;

    if experience == "yes" && age >=  40 {
        incentive = 1_560_000.0;   
   } else if experience == "yes" && age >= 30 && age <= 39 {
        incentive = 1_480_000.0;
   } else if experience == "yes" && age <= 29 {
        incentive = 1_300_000.0;
   } else if experience == "no" {
        incentive = 100_000.0;
     
    } else {
        incentive = 0.0;
        println!("Sorry, we don't have incentives for you.");
   }
   println!("your employee incentive is: {}naira", incentive);
} 
  
