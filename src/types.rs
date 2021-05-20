/*
Primitive types:
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples
    Arrays
*/

pub fn run() {
    println!("\n");
    // default is i32
    let x = 1;

    // default is f64
    let y = 1.0;

    // explicit type
    let z: i64 = 0985401701235;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max f64: {}", std::f64::MAX);

    // boolean
    let is_active = true;
    if is_active {
        println!("{:?}", (x, y, z, is_active));

        // character
        let a1 = 'a';
        if a1 != 'a' {
            println!("Won't be printed!");
        } else {
            let smiley = '\u{1F600}';
            println!("{}", smiley);
        }
    }
}
