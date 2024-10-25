fn main() {
    let p: f64 = 520_000_000.0;
    let t: f64 = 5.0;
    let r: f64 = 10.0;
    let a: f64= p*(1.0+(r/100.0))*t;
    let ci: f64 = a-p;
    println!("Amount is {a}");
    println!("Compound Interest is {ci}");
}