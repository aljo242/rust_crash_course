mod arrays;
mod conditionals;
mod print;
mod strings;
mod tuples;
mod types;
mod vars;
mod vectors;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main() {
    // basic printing
    print::run();

    // variables and declarations
    vars::run();

    // basic type system
    types::run();

    // strings libary
    strings::run();

    // tuple type
    tuples::run();

    // array type
    arrays::run();

    //vectors types
    vectors::run();

    // conditional statments
    conditionals::run();

    // loops
    loops::run();

    functions::run();

    pointer_ref::run();

    structs::run();

    enums::run();

    cli::run();
}
