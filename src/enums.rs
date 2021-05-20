

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // perform action depending on info
    // match is like a switch statment
    match m {
        Movement::Up => println!("Moving up!"),
        Movement::Down => println!("Moving down!"),
        Movement::Left => println!("Moving left!"),
        Movement::Right => println!("Moving right!"),
    }
}

pub fn run() {
    println!("\n");
    move_avatar(Movement::Right);
    move_avatar(Movement::Up);
    move_avatar(Movement::Left);
    move_avatar(Movement::Down);

}