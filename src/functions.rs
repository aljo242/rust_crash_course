pub fn run() {
    println!("\n");
    greeting("Hello", "Alex");

    // bind function value to variables
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // function closures 
    let add_nums = |n1: i32, n2: i32|
        n1 + n2 + sum;
    println!("closure sum: {}", add_nums(1, 2));
}


fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}