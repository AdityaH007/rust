// STRUCTS  
struct Color (i32, i32, i32); // tuple struct

struct  User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
 fn main()
 {
     let mut email = String::from("helli.com");
     let mut username = String::from("helli");
   let user1=  build_user(email , username);
    print!("{}", user1.email);

    let black = Color(0,0,0);
    println!("{}", black.0);
 }

fn build_user(email:String , username:String) -> User
{
    User {
        email: email,
        username , // field init shorthand syntax
        active: true,
        sign_in_count: 1,
    }
}
