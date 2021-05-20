// variables hold primitive data or references to data
// variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    println!("\n");
    let name = "Alex"; // default "const"
    let mut age = 23; // have to say "mut" to be mutable
    age += 1;
    println!("My name is: {}\nI am {} years old", name, age);

    // defining constants
    const ID: i32 = 001;
    println!("const ID: {}", ID);

    // assign to multiple variables at once
    let (a, b, c) = (1, 2, 3);
    let d = a + b + c;
    println!("{} + {} + {} = {}", a, b, c, d);
}
