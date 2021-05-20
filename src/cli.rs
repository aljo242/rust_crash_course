pub fn run() {
    println!("\n");
    let args: Vec<String> = std::env::args().collect(); // get args as vec
    let command = args[1].clone();
    if command == "test" {
        println!("TEST RUN");
    }

    println!("Args {:?}", args);
}