//ownership
//




fn main() {

    let _s = "hello"; // s comes into scope here

    // String type from string literal for storing in heap
    let mut  _sone = String::from("HellO");
    _sone.push_str("World");
    println!("{}", _sone);



//DIFFERENCE  
let x =5;
let y = x; //possible cause x is stored in stack and is a primmitive type

let s1 = String::from("hello");
let s2 = s1; //not possible cause s1 is stored in heap and is a non primmitive type

//println!("{}", s1); //error, value moved to s2 
//Using clone for deep copy
let s3 = s2.clone();
println!("{}",s3);


} // s gets out of scope here 
