// Primitive str = Immutable fixed-length string somethwere in memory
// string = growable, heap-allocated data-structure - use when modifying
// so again we have this separation of mutable and immutable data

fn print_string_info(string: &str) {
    if !string.is_empty() {
        println!("value: {}, Len: {}", string, string.len());
    }

    let keyword = "World";
    if string.contains(keyword) {
        println!(
            "This string contains the substring: {}! Modifying it...",
            keyword
        );
        let new_string = string.replace(keyword, "There!");
        println!("New string is: {}", new_string);
    }
}

pub fn run() {
    println!("\n");
    let primitive_str = "Hello";
    let mut lib_string = String::from(primitive_str);

    print_string_info(lib_string.as_str());

    // adding a string to it
    lib_string.push_str(" World!");
    print_string_info(lib_string.as_str());

    // loop through string by whitespace
    for word in lib_string.split_whitespace() {
        println!("{}", word);
    }

    // create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('c');
    println!("{}", s);

    // assert macro to see if equal
    assert_eq!(s, "abc");
}
