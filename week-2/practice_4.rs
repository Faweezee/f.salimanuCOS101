fn main() {
    let p: f64 = 1_000.0;
    let r: f64 = 1.0;
    let t: f64 = 2.0;

    let a = p*(1.0 + (r/100.0)) * t;
    println!("Amount is {a}");
    let si = a -p;
    println!("Simple Interest is {si}");
}