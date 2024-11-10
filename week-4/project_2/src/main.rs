use std::io;

fn main() {

    let mut in2 = String::new();
    let mut in3 = String::new();
    let mut inct = 000;

   
    println!("Enter your age: ");
    io::stdin().read_line(&mut in2).expect("Invalid input");
    let age: u32 = in2.trim().parse().expect("Invalid input");
    
    println!("Are you experirienced: (true/false)");
    io::stdin().read_line(&mut in3).expect("Invalid input");
    let experience: bool = in3.trim().parse().expect("Invalid input");
    


    if experience == true
    {
        if age >= 40
        {
            inct = 1_560_000;
        }
        else if age >= 30 && age< 40
        {
            inct = 1_480_000;
        }
        else if age < 28 
        {
            inct = 1_300_000;
        }
        else{
            inct = 0;
        }

    }
    else if experience == false{
        inct = 100_000;
    }

    println!("Your incentive is {inct}");
}
