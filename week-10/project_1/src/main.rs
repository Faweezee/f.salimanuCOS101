struct Laptop{
    model: String,
    price: u32,
    amount: u32
}

impl Laptop{
    fn sell(&mut self, qty: u32)
    {
        self.amount -= qty; 
    }
}

fn main()
{
    let mut dells = Laptop{model:String::from("DELL"), price: 850000, amount: 4};
    let mut hps = Laptop{model:String::from("HP"), price: 650000, amount: 10};
    let mut ibms = Laptop{model:String::from("IBM"), price: 755000, amount: 6};
    let mut toshibas = Laptop{model:String::from("TOSHIBA"), price: 550000, amount: 10};
    println!("\n\n      ------  Mr. OGBEIFUNA'S INVENTORY :  ------");  
    display_struct(&dells);
    display_struct(&hps);
    display_struct(&ibms);
    display_struct(&toshibas);

    let cost_for_three_all = calc_cost(&mut dells, &mut hps, &mut ibms, &mut toshibas, 3,3,3,3);
    
    println!("Cost for customer to buy 3 Dells, 3 HPs, 3 Toshibas and 3 IBMs is {}", cost_for_three_all);
    
    //let cost_for_three_all = calc_cost(&mut dells, &mut hps, &mut ibms, &mut toshibas, 3,3,3,3);

    /*println!("\n\n      ------  Mr. OGBEIFUNA'S INVENTORY :  ------");  
    display_struct(&dells);
    display_struct(&hps);
    display_struct(&ibms);
    display_struct(&toshibas);*/
}   

fn display_struct(lapt: &Laptop)
{
    println!("MODEL: {},\nPRICE: {},\nAMOUNT AVAILABLE: {}\n", lapt.model, lapt.price, lapt.amount);
}

fn calc_cost(lapt_dell: & mut Laptop,lapt_hp: &mut Laptop,lapt_ibm: &mut Laptop,lapt_tosh: &mut Laptop,dell_amt: u32, hp_amt: u32, ibm_amt: u32, tosh_amt: u32) -> u32
{
    let mut cost: u32;
    if lapt_dell.amount >= dell_amt && lapt_hp.amount >= hp_amt && lapt_ibm.amount >= ibm_amt && lapt_tosh.amount >= tosh_amt
    {
        lapt_dell.sell(dell_amt);
        lapt_hp.sell(hp_amt);
        lapt_ibm.sell(ibm_amt);
        lapt_tosh.sell(tosh_amt);
        cost = (lapt_dell.price * dell_amt) + (lapt_hp.price * hp_amt) + (lapt_tosh.price * tosh_amt) + (lapt_ibm.price * ibm_amt);
    }
    else
    {
        println!("INSUFFICIENT AMOUNT IN INVENTORY FOR SALE");
        cost = 0;
    }

    return cost;
    
}