use std::mem;

pub fn run() {
    println!("\n");
    let mut numbers: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];

    // print array with debug print
    println!("{:?}", numbers);
    println!("Modifying vector...");
    numbers.push(8);
    numbers.push(9);

    // loop and mutate values
    for num in numbers.iter_mut() {
        *num *= 2;
    }
    println!("{:?}", numbers);

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice of array
    let slice: &[u32] = &numbers[1..6];
    println!("Slice: {:?}", slice);
}
