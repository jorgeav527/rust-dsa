fn main() {
    let my_vec: Vec<i32> = vec![7, 12, 9, 4, 11];
    let mut min_val: i32 = my_vec[0];

    for &i in &my_vec {
        if i < min_val {
            min_val = i
        }
    }
    println!("Lowest value {:?}", min_val)
}
