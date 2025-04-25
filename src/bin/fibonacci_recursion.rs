fn fibo(number_1: u32, number_2: u32, count: &mut u32, max_count: u32) {
    if *count < max_count {
        let new_number = number_1 + number_2;
        println!("{:?}", new_number);
        // update the value count dereferenced
        *count += 1;
        fibo(number_2, new_number, count, max_count);
    }
}

fn main() {
    println!("{:?}", 0);
    println!("{:?}", 1);
    let mut count = 2;
    let max_count = 20; // including the initial 0 and 1

    fibo(0, 1, &mut count, max_count);
}
