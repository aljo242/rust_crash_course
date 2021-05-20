// arrays are fixed lists where elements are the same type
// stack allocated

pub fn run() {
    println!("\n");
    let numbers: [u32; 5] = [1, 2, 3, 4, 5];

    // print array with debug print
    println!("{:?}", numbers);

    // print array by values iterating through
    for num in &numbers {
        println!("{}", num);
    }

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice of array
    let slice: &[u32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
