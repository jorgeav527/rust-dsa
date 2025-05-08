fn main() {
    let mut arr: Vec<i32> = vec![7, 3, 9, 12, 11, 1];
    println!("Original array: {:?}", arr);
    
    // Get the length of the vector
    let n = arr.len();
    
    for i in 0..n-1 {
        println!("outer loop {}", i);
        println!("list to loop outer: {:?}", 0..n-1);
        
        let mut swapped = false;
        
        for j in 0..n-i-1 {
            println!("inner loop {}", j);
            println!("list to loop inner: {:?}", 0..n-i-1);
            
            if arr[j] > arr[j + 1] {
                println!("actual state: {} {}", arr[j], arr[j + 1]);
                // Swap the elements
                arr.swap(j, j + 1);
                println!("new state: {} {}", arr[j], arr[j + 1]);
                swapped = true;
            }
        }
        
        println!("Array after iteration {}: {:?}", i, arr);
        
        // If no swapping occurred in this iteration, the array is already sorted
        if !swapped {
            println!("Early termination: Array is sorted");
            break;
        }
    }
    
    println!("Sorted array: {:?}", arr);
}