fn fibo(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}

fn main() {
    for i in 0..20 {
        println!("{}", fibo(i));
    }
}
