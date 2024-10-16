use std::io;

fn main()
{
    println!("Guess the number!");

    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read the input");

    println!("You guessed: {}", guess);

    // placeholders for variables
    let  x =5;
    println!("The value of x is: {}", x); //this is a placeholder

    //println!("The value of x is {}"); this gives an error because there is no placeholder for x
    
}
