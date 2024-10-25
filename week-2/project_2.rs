fn main() {
    let mut toshiba:f64 = 450000.0;
    let mut mac: f64 = 1500000.0;
    let mut hp: f64 = 750000.0;
    let mut dell: f64 = 2850000.0;
    let mut acer:f64 = 250000.0;
    let tosh_qty: f64 = 2.0;
    let mac_qty: f64 = 1.0;
    let hp_qty: f64 = 3.0;
    let dell_qty: f64 = 3.0;
    let acer_qty: f64 = 1.0;


    toshiba = toshiba * tosh_qty;
    mac = mac * mac_qty;
    hp = hp * hp_qty;
    dell = dell * dell_qty;
    acer = acer * acer_qty;
    
    let sum: f64 = toshiba + dell + acer + mac + hp;
    let avg: f64 = sum / (tosh_qty+dell_qty+acer_qty+mac_qty+hp_qty);
    println!("The sum of all sales in the sales record is {sum}");
    println!("The average of all sales in the sales record is {avg}");
}