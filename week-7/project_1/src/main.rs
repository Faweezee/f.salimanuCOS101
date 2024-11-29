use std::io;

fn get_input(side: &str)->f64
{
    let mut input = String::new();
    println!("Enter the {side}: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let sidee: f64 = input.trim().parse().expect("Invalid Input");
    return sidee;
}

fn main()
{
    println!("\n    AREA CALCULATOR FOR SHAPES");
    println!("SHAPE CODES:\n1.  TRAPEZIUM\n2.  RHOMBUS\n3.  PARALLELOGRAM\n4.  CUBE\n5.  CYLINDER");
    println!("Input the number for the shape you'd like to calculate the are for.\nExample, for A cube input 4.\n");
    let choice = get_input("shape code for your chosen shape");


    if choice == 1.0
    {
        trap();
    }
    else if choice == 2.0
    {
        rhom();
    }
    else if choice == 3.0
    {
        para();
    }
    else if choice == 4.0
    {
        cube();
    }
    else if choice == 5.0
    {
        cyl();
    }
    else 
    {
        println!(" Invalid Input,\nAn ERROR has occured");
    }
}

fn trap()
{
    println!("You have selected TRAPEZIUM");
    let base1 = get_input("length of the first base (in cm)");
    let base2 = get_input("length of the second base (in cm)");
    let hgh = get_input("height (in cm)");
    let area = (hgh/2.0) * (base1 + base2);
    println!("The area of the trpezium is {area}");
}

fn rhom()
{
    println!("You have selected RHOMBUS");
    let dia1 = get_input("length of the first diagonal (in cm)");
    let dia2 = get_input("length of the second diagonal (in cm)");
    let area = 0.5 * dia1 * dia2;
    println!("The area of the rhombus is {area}");
}

fn para()
{
    println!("You have selected PARALLELOGRAM");
    let base = get_input("length of the base (in cm)");
    let hgh = get_input("height/altitude (in cm)");
    let area = base * hgh;
    println!("The area of the parallelogram is {area}");
}

fn cube()
{
    println!("You have selected CUBE");
    let base1 = get_input("length of a side  of the cube(in cm)");
    let area = 6.0 * base1.powf(2.0);
    println!("The area of the cube is {area}");
}

fn cyl()
{
    println!("You have selected CYLINDER");
    let rad = get_input("length of the radius of the circular face of the cylinder (in cm)");
    let hgh = get_input("height of the cylinder (in cm)");
    let area = (22.0/7.0) * rad.powf(2.0) * hgh;
    println!("The area of the cylinder is {area}");
}