pub fn run() {
    println!("\n");
    let age: u8 = 18;
    let mut check_id = false;
    let mut done = false;

    // if else
    while done != true {
        if age >= 21 && check_id {
            println!("omg rly old");
            done = true;
        } else if age < 21 && check_id {
            println!("young fart");
            done = true;
        } else {
            println!("hi, can I see your id??");
            check_id = true;
        }
    }
}
