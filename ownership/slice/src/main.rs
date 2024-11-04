use std::{array::from_ref, usize};

fn main() {
    println!("Hello, world!");

    let mut s = String::from("HELLO RUST");
    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
        
    }
    s.len()
}
