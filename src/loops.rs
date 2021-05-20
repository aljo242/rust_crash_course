pub fn run() {
    println!("\n");

    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("{}", count);
        
         if count > 5 {
             break;
         }
    }

    println!("Starting fizzbuzz using while loop...");
    // while loop fizzbuzz
    while count <= 20 {
        if count % 15 == 0 {
            println!("{}", "fizzbuzz");
        } else if count % 3 == 0 {
            println!("{}", "fizz");
        } else if count % 5 == 0 {
            println!("{}", "buzz");
        } else {
            println!("{}", count);
        }
        
        count += 1;
    }

    println!("Starting fizzbuzz using for range loop...");
    // for range loop
    for x in 0..10 {
        if x % 15 == 0 {
            println!("{}", "fizzbuzz");
        } else if x % 3 == 0 {
            println!("{}", "fizz");
        } else if x % 5 == 0 {
            println!("{}", "buzz");
        } else {
            println!("{}", x);
        }
    }
}