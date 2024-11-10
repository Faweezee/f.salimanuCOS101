use std::io;

fn main() {
    let mut in1 = String::new();
    let mut in2 = String::new();
    let mut in3 = String::new();
    println!("Enter a; the coefficient of x squared: ");
    io::stdin().read_line(&mut in1).expect("Invalid input");
    let a: f32 = in1.trim().parse().expect("Invalid input");

    println!("\nEnter b; the coefficient of x: ");
    io::stdin().read_line(&mut in2).expect("Invalid input");
    let b: f32 = in2.trim().parse().expect("Invalid input");

    println!("\nEnter c; the constant: ");
    io::stdin().read_line(&mut in3).expect("Invalid input");
    let c: f32 = in3.trim().parse().expect("Invalid input");

    let  d: f32 = b.powf(2.0) - (4.0*a*c);
    if d ==0.0
    {
        let x: f32 = (d.sqrt() -b)/ (2.0*a);
        println!("\n\nThe discriminant,d is {d}.\nTherefore the equation has two roots that are real and equal.\nx = {}", x);
    }
    else if d>0.0 
    {
        let x1: f32 = (d.sqrt() -b)/ (2.0*a);
        let x2: f32 = (-(d.sqrt()) -b)/ (2.0*a);
        println!("\n\nThe discriminant,d is {d}.\nTherefore the equation has two roots that are real and distinct.\nx = {}        OR\nx = {}", x1,x2);
    }
    else{
        println!("\n\nThe discriminant,d is {d}.\nTherefore the equation has imaginary roots.");
    }
}
