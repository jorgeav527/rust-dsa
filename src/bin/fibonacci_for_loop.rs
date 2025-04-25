fn main() {
    // Initialize variables number_1 and number_2 (0, 1)
    let mut number_1: u32 = 0;
    let mut number_2: u32 = 1;
    println!("{:?}", number_1);
    println!("{:?}", number_2);

    for _ in 1..19 {
        // Calculate the next Fibonacci number
        let new_number = number_1 + number_2;
        // Update the variables for the next iteration
        number_1 = number_2;
        number_2 = new_number;
        println!("{:?}", new_number);
    }
}
