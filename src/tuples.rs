// tuples group together values of different types
// max of 12 elements

pub fn run() {
    println!("\n");
    let person: (&str, &str, u8) = ("alex", "pittsburgh", 24);
    println!("{} lives in {} and is {}", person.0, person.1, person.2);
}
