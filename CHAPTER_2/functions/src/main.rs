



fn main() {
    println!("FUNCTIONS");
    another_function(5, 6);

    let x =5;
    let y = {
        let x = 3;
        x + 1
    }; //its an expression
       //
    println!("The value of y is:{}" , y);
    println!("{}", foo());
    println!("{}", anotherfoo(9));
}

fn another_function(x:i32 , y:u32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let z = 23;
    println!("The value of z is: {}", z);
}

//function with return value

fn foo()->u32
{
    println!("This is foo function");
    5
}

fn anotherfoo(i:i32)->i32
{
    i+1
}
