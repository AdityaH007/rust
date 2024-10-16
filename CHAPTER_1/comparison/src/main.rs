extern crate rand;

use core::num;
use std::{io, u32};

use std::cmp::Ordering;

use rand::Rng;

fn main()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("the scecret number is: {}",secret_number);
loop{
    println!("enter your guess");

    let mut guess = String::new();

    //looping

    

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {}",guess);
//type conversion
let guess: u32 = match guess.trim().parse(){
    Ok(num) => num,
  Err(_) => continue,
};

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal =>{ println!("you win");break;}
        
    }
}
}
