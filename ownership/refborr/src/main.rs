// References and borrowing 


fn main()
{
    let s1= String::from("Hello");

    let len = calculate_lenght(&s1);

    println!("The length of '{}' is {}", s1, len);

    //mutable refs
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    //multiple scopes
    let r1 = &mut s;
    {
        let r2 = &mut s;
        println!("{}", r2);
    }
}


fn calculate_lenght(s: &String) -> usize
{
    s.len()
}

fn change(s: &mut String)
{
    s.push_str(", World");
}
