use std::io;

fn main()
{
    println!("SIBLING INFORMATION SYSTEM");
    
    println!("\nHow many siblings do you have: ");
    let mut nsib = String::new();
    io::stdin().read_line(&mut nsib).expect("INVALID INPUT");
    let nsib: u32 = nsib.trim().parse().expect("INVALID INPUT");

    for x in 0..nsib {
        let mut firstname = String::new();
        let mut gender = String::new();
        let mut cor = String::new();
        let mut age = String::new();
        println!("\n\nEnter the first name of sibling {}: ", x+1);
        io::stdin().read_line(&mut firstname).expect("INVALID INPUT");
        firstname = firstname.trim().to_string();

        println!("Enter the age of sibling {}: ", x+1);
        io::stdin().read_line(&mut age).expect("INVALID INPUT");
        let age: u32 = age.trim().parse().expect("INVALID INPUT");

        println!("Enter the gender of sibling {}:  (m/f)",x+ 1 );
        io::stdin().read_line(&mut gender).expect("INVALID INPUT");
        gender = gender.to_lowercase().trim().to_string();

        println!("Enter the country of residence of sibling {}:  ",x+ 1 );
        io::stdin().read_line(&mut cor).expect("INVALID INPUT");
        cor = cor.trim().to_string();

        if age >= 18{
            let mut noc= String::new();
            
            let mut relstat = String::new();
            println!("Enter relationship status of sibling {}: (married/single/relationship)", x+1);
            io::stdin().read_line(&mut relstat).expect("INVALID INPUT");
            relstat = relstat.trim().to_string().to_lowercase();
            
            

            if relstat == "married"
            { 
                println!("How many children do you have: (0 if none)");
                io::stdin().read_line(&mut noc).expect("INVALID INPUT");
                let noc: u32 = noc.trim().parse().expect("INVALID INPUT");

                if noc > 0 
                {
                    for y in 0..noc {
                        let mut cname = String::new();
                        let mut cage = String::new();
                        let mut cskl = String::new();
                        println!("What is the name of child {} of sibling {}:  ", y+1,x+1);
                        io::stdin().read_line(&mut cname).expect("INVALID INPUT");
                        cname = cname.trim().to_string();

                        println!("Enter the age of child {} of sibling {}:  ", x+1,y+1);
                        io::stdin().read_line(&mut cage).expect("INVALID INPUT");
                        let cage: u32 = cage.trim().parse().expect("INVALID INPUT");

                        println!("What is the name the school or daycare of child {} of sibling {}:  ", x+1,y+1);
                        io::stdin().read_line(&mut cskl).expect("INVALID INPUT");
                        cskl = cskl.trim().to_string();
                    }
                }

                let mut ccor = String::new();
                println!("Enter the city of residence of sibling {}&family:  ",x+ 1 );
                io::stdin().read_line(&mut ccor).expect("INVALID INPUT");
                ccor = ccor.trim().to_string();

                println!("\n{firstname} and family live in {ccor}");
            }
            else if relstat == "single"
            {
                
                
                let mut jbstat = String::new();
                println!("Enter employment status of sibling {}: (student/employed)", x+1);
                io::stdin().read_line(&mut jbstat).expect("INVALID INPUT");
                jbstat = jbstat.trim().to_string().to_lowercase();
                    


                if jbstat == "student"
                {
                    let mut uni = String::new();
                    let mut cofs = String::new();
                    let mut yofs = String::new();
                    let mut hoa = String::new();

                    println!("Enter the name of the university sibling {} attends: ", x+1);
                    io::stdin().read_line(&mut uni).expect("INVALID INPUT");
                    uni = uni.trim().to_string();

                    println!("Enter the course of study sibling {} : ", x+1);
                    io::stdin().read_line(&mut cofs).expect("INVALID INPUT");
                    cofs = cofs.trim().to_string();

                    println!("Enter the year of study of sibling {}: ", x+1);
                    io::stdin().read_line(&mut yofs).expect("INVALID INPUT");
                    let yofs: u32 = yofs.trim().parse().expect("INVALID INPUT");

                    println!("Is sibling {} studying in home country or abroad: (home/abroad)", x+1);
                    io::stdin().read_line(&mut hoa).expect("INVALID INPUT");
                    hoa = hoa.trim().to_string();
                    let mut cofss = String::new();

                    if hoa == "abroad"{
                        println!("What country is sibling {} studying: ", x+1);
                        io::stdin().read_line(&mut cofss).expect("INVALID INPUT");
                        cofss = cofss.trim().to_string();
                    }
                    else {
                        cofss = cor;
                    }

                    println!("{firstname} is studying {cofs} in {uni} in the country {cofss} with the year of study {yofs}");

                }
                else if jbstat == "employed"
                {
                    let mut jbstatnat = String::new();
                    println!("Is the job remote/on-site/hybrid: ");
                    io::stdin().read_line(&mut jbstatnat).expect("INVALID INPUT");
                    jbstatnat = jbstatnat.trim().to_string().to_lowercase();

                    if jbstatnat == "onsite" || jbstatnat == "on site" || jbstatnat == "on-site"
                    {
                        let mut jbttl = String::new();
                        let mut cmpny = String::new();
                        let mut indstry = String::new();
                        println!("What is the name of the company worked at: " );
                        io::stdin().read_line(&mut cmpny).expect("INVALID INPUT");
                        cmpny = cmpny.trim().to_string();
                        println!("What is the job title: ");
                        io::stdin().read_line(&mut jbttl).expect("INVALID INPUT");
                        jbttl = jbttl.trim().to_string();
                        println!("What is the industry sector of the company worked at: ");
                        io::stdin().read_line(&mut indstry).expect("INVALID INPUT");
                        indstry = indstry.trim().to_string();

                        println!("{firstname} works with the company {cmpny} in the industry sector {indstry} and their job title is {jbttl}");
                    }

                }

            }
            else if relstat == "relationship"
            {
                let mut reldur = String::new();
                let mut prtnrname = String::new();
                let mut lif = String::new();
                println!("How long has the relationship of {firstname} lasted for :   (in years)");
                io::stdin().read_line(&mut reldur).expect("INVALID INPUT");
                let reldur: f32 = reldur.trim().parse().expect("INVALID INPUT");
                println!("What is the first name of {firstname}'s partner: ");
                io::stdin().read_line(&mut prtnrname).expect("INVALID INPUT");
                prtnrname = prtnrname.trim().to_string();
                println!("Does {firstname} live with their partner: (y/n)" );
                io::stdin().read_line(&mut lif).expect("INVALID INPUT");
                lif = lif.trim().to_string().to_lowercase();
                if lif == "y" || lif == "yes"
                {
                    let mut ccorr = String::new();
                    println!("Enter the city of residence of {firstname} and partner:  ");
                    io::stdin().read_line(&mut ccorr).expect("INVALID INPUT");
                    ccorr = ccorr.trim().to_string();
                    println!("{firstname} lives with {prtnrname} in the city {ccorr}");
                }

                
            }

            
        }
        else if age < 18
        {
            let mut waecstat = String::new();
            let mut secskl = String::new();
            let mut fingrd = String::new();
            let mut yorc = String::new();
            let mut clss = String::new();
            let mut waecps = String::new();
            let mut waecyr = String::new();

            println!("Has {firstname} completed the West African Examinations Council (WAEC) Exam:   (y/n");
            io::stdin().read_line(&mut waecstat).expect("INVALID INPUT");
            waecstat = waecstat.trim().to_string().to_lowercase();
            if waecstat == "y" || waecstat == "yes"
            {
                println!("Enter the secondary school name that {firstname} attended:   ");
                io::stdin().read_line(&mut secskl).expect("INVALID INPUT");
                secskl = secskl.trim().to_string();

                println!("Enter the final grade that {firstname} attained:   ");
                io::stdin().read_line(&mut fingrd).expect("INVALID INPUT");
                fingrd = fingrd.trim().to_string();

                println!("Enter the year that {firstname} completed secondary school:   ");
                io::stdin().read_line(&mut yorc).expect("INVALID INPUT");
                yorc = yorc.trim().to_string();


                println!("{firstname} completed scondary school from {secskl} and completed the WAEC exam with a final grade of {fingrd} in the year {yorc}");

            }
            else {
                println!("What is {firstname}'s current class level: ");
                
                io::stdin().read_line(&mut clss).expect("INVALID INPUT");
                clss = clss.trim().to_string();
                println!("Does {firstname} plan on writing the West African Examinations Councils Exam:   (y/n)");
                
                io::stdin().read_line(&mut waecps).expect("INVALID INPUT");
                waecps = waecps.trim().to_string();
                if waecps == "y" || waecstat == "yes"
                {
                    println!("When does {firstname} plan on writing the West African Examinations Councils Exam:   ");
                
                    io::stdin().read_line(&mut waecyr).expect("INVALID INPUT");
                    waecyr = waecyr.trim().to_string();
                    println!("{firstname} plans on writing the WAEC exam in the year {waecyr}");
                }
            }

        }
    }
}