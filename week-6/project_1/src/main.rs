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

    println!("\n\n\nNOTE: USE THE CODES FOR THE MEALS");
    println!("");
    println!("What would you like to order? (P/ F/ A/ E/ W)\n ORDER: ");
    let mut in1 = String::new();
    io::stdin().read_line(&mut in1).expect("INVALID INPUT");
    let ord = in1.trim().to_lowercase().to_string();

    println!("How many would you like to order (quantity):  ");
    let mut in2 = String::new();
    io::stdin().read_line(&mut in2).expect("INVALID INPUT");
    let qty: f32 = in2.trim().parse().expect("INVALID INPUT");

    if ord == "p" {
        cost = pP * qty;
    }
    else if ord == "f" {
        cost = pF * qty;
    }
    else if ord == "a" {
        cost = pA * qty;
    }
    else if ord == "e" {
        cost = pE * qty;
    }
    else if ord == "w" {
        cost = pW * qty;
    }
    else{
        println!("INVALID INPUT");
    }


    if cost > 10_000.0 {
        cost = cost * 0.95;
    }

    println!("The cost of your order is {cost}");

}