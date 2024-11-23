fn main() {
    let a: i32 = 2; //bit representation is 10
    let b: i32 = 3; // bit representation is 11

    let mut result: i32;

    println!("");

    result = a & b;
    println!("(a & b) -> {}", result);

    result = a | b;
    println!("(a | b) -> {}", result);

    result = a ^ b;
    println!("(a ^ b) -> {}", result);

    result = !b;
    println!("(!b) -> {}", result);

    result = a << b;
    println!("(a << b) -> {}", result);

    result = a >> b;
    println!("(a >> b) -> {}", result);
}
