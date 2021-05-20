pub fn run() {
    println!("\n");

    let c = Color{red: 255, green: 0, blue: 0};
    println!("Color struct: {} {} {}", c.red, c.green, c.blue);

    let c_tuple = ColorTuple(255, 0, 0);
    println!("Color tuple: {} {} {}", c_tuple.0, c_tuple.1, c_tuple.2);

    let mut p = Person::new("Alex", "Johnson");
    println!("Person: {}", p.full_name());

    p.set_last_name("Wang-Johnson");
    println!("Person: {}", p.full_name());

    let p_from_tup = p.to_tuple();
}


struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        };
    }

    // get full name
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    } 

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        return (self.first_name, self.last_name);
    }

}