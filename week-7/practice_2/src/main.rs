use std::io;

fn checker()
{
    let mut input = String::new();
    println!("Enter a character: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let chr: char = input.trim().parse().expect("Invalid input");


    if chr>= '0' && chr<='9'
    {
        println!("Character '{chr}' is a digit");
    }
    else{
        println!("Character '{chr}' is not a digit");
    }
}


fn main()
{
    println!("Welcome! This program checks whether a character variable
contains a digit or not");
    checker();
}