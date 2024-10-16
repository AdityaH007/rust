fn main() {
    println!("_________________");
    println!("| COMPOUND TYPES|");
    println!("-----------------");

    println!("Tuple");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {:?}", tup);

    println!("Array");
    let a = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", a);
}
