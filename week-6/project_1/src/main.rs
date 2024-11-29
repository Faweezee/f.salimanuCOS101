use std::io;

fn main() {
    let pP = 3200.0;
    let pF = 3000.0;
    let pA = 2500.0;
    let pE = 2000.0;
    let pW = 2500.0;
    
    let mut cost: f32 = 0.0;
    println!("\n\n                      Menu                                    Price ( N )");
    println!("");
    println!("      P = Poundo Yam/Edinkaiko Soup                           -3,200");
    println!("      F = Fried Rice & Chicken                                -3,000");
    println!("      A = Amala & Ewedu Soup                                  -2,500");
    println!("      E = Eba & Egusi Soup                                    -2,000");
    println!("      W = White Rice & Stew                                   -2,500");

    loop {
        println!("\n\n\nNOTE: USE THE CODES FOR THE MEALS");
        println!("");
        println!("What would you like to order? (P/ F/ A/ E/ W)\n ORDER: ");
        let mut in1 = String::new();
        io::stdin().read_line(&mut in1).expect("INVALID INPUT");
        let ord = in1.trim().to_lowercase().to_string();

        println!("How many units would you like to order (quantity):  ");
        let mut in2 = String::new();
        io::stdin().read_line(&mut in2).expect("INVALID INPUT");
        let qty: f32 = in2.trim().parse().expect("INVALID INPUT");
        if ord == "p" {
            cost = cost + (pP * qty);
        }
        else if ord == "f" {
            cost =cost + (pF * qty);
        }
        else if ord == "a" {
            cost  =cost + (pA * qty);
        }
        else if ord == "e" {
            cost = cost + (pE * qty);
        }
        else if ord == "w" {
            cost =cost + (pW * qty);
        }
        else{
            println!("INVALID INPUT");
        }    

        println!("Would you like to order anything else?  (y/ n)");
        let mut in3 = String::new();
        io::stdin().read_line(&mut in3).expect("INVALID INPUT");
        let rept = in3.trim().to_lowercase().to_string();

        if rept == "y" || rept == "yes"
        {
            
        }
        else {
            break;
        }

    }

    if cost > 10_000.0 {
        cost = cost * 0.95;
    }

    println!("The cost of your order is N{cost}");

}