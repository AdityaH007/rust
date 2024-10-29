//dangline references

fn main()
{
    let ref_to_nothing = dangle();

}

fn dangle() -> &String 
{
    let s= String::from("Hello");
    &s //we return a ref to string s
} //here s goes out of scope and is dropped , its memory goes away
