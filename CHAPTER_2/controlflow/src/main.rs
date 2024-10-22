fn main()
{
    let num =3;

    if (num < 5)
    {
        println!("condition was true");
    }
    else
    {
        println!("condition was false");
    }

    //while loop

    let mut x =3;
    while x !=0 {
        println!("{}!", x);
        x -=1;
    }

    //for loop

    for i in 0..5 {
        println!("{}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
