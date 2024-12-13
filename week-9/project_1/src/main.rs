use std::io::Write;
use std::io::Read;

fn main()
{
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo", "Williams"];
    let nonalc = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    file.write_all("    Lager           Stout           Non-Alcoholic\n\n".as_bytes()).expect("write failed");

    let mut count = 0;
    loop
    {
        
        let index = count as usize;

        if index < lager.len()
        {
            let indx = index as usize;
            file.write_all(lager[indx].as_bytes()).expect("write failed");
            file.write_all("                ".as_bytes()).expect("write failed");
        }

        if index < stout.len()
        {
            let indx = index as usize;
            file.write_all(stout[indx].as_bytes()).expect("write failed");
            file.write_all("                ".as_bytes()).expect("write failed");
            
        }
        else
        {
            
            file.write_all("                ".as_bytes()).expect("write failed");
            file.write_all("                ".as_bytes()).expect("write failed");
        
        }

        if index < nonalc.len()
        {
            let indx = index as usize;
            file.write_all(nonalc[indx].as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes()).expect("write failed");

        }
        else
        {
            file.write_all("\n".as_bytes()).expect("write failed");

        }

        if count == lager.len()-1
        {
            break;
        }
        else{
            count += 1;
        }
    }

    let mut contents = String::new();
    let mut files = std::fs::File::open("drinks.txt").expect("opening failed");
    files.read_to_string(&mut contents).expect("read failed");
    println!("{}", contents);
}