use std::io::Write;
use std::io::Read;

fn main()
{
    let mut file = std::fs::File::create("pau-smis.txt").expect("create failed");
    file.write_all("                                            PAU SMIS".as_bytes()).expect("write failed");
    file.write_all("\n\n    STUDENT NAME                    MATRIC. NUMBER                  DEPARTMENT                          LEVEL\n".as_bytes()).expect("write failed");
    let mut cont = "y";
    let mut names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let mut matricno = vec!["ACC10211111", "ECO10110101","CSC10328828", "EEE11020202", "MEE10202001"];
    let mut department = vec!["Accounting","Economics","Computer", "Electrical", "Mechanical"];
    let mut level = vec!["300","100","200","200","100"];

    for i in 0..names.len()
    {
        let index = i as usize;
        file.write_all(names[index].as_bytes()).expect("write failed");
        file.write_all("                        ".as_bytes()).expect("write failed");
        file.write_all(matricno[index].as_bytes()).expect("write failed");
        file.write_all("                        ".as_bytes()).expect("write failed");
        file.write_all(department[index].as_bytes()).expect("write failed");
        file.write_all("                        ".as_bytes()).expect("write failed");
        file.write_all(level[index].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    

    let mut contents = String::new();
    let mut files = std::fs::File::open("pau-smis.txt").expect("read failed");
    files.read_to_string(&mut contents).expect("read failed");
    println!("{}",contents);
    

}