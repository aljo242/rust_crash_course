pub fn run() {
    println!("\n");
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("{} {} {}: {}", "Printing", "a", "number", 1);

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.",
        "Alex", "Pitt", "code"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}.",
        name = "Alex",
        activity = "soccer"
    );

    // placeholder traits
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    //placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("{} + {} = {}", 10, 10, 10 + 10);
}
