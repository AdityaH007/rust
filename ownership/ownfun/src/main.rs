//Ownership and function

fn main() {
    println!("Ownership and function");

    let s = String::from("HELLO"); // s comes into scope
                                   
    takes_ownership(s); //value of s is moved to func 
                        //hence its no longer valid 

    let x =5;
    make_copy(x); //x would move into func
                  //but it would still be valid


} // x gets dropped here but s was already moved to func


fn takes_ownership(some_string: String) //some string comes into scope
{
    println!("{}",some_string);
}// some_string goes out of scope and is dropped
 

fn make_copy(some_integer: i32) //some integer comes into scope   
{
    println!("{}",some_integer);
    
}// some_integer goes out of scope and is dropped:
