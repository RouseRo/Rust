fn main() {
    let tuple: (i32, f64, &str) = (42, 3.14, "Hello");
    
    // Accessing tuple elements
    let (x, y, z) = tuple; // Destructuring
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // Accessing using index
    println!("First element: {}", tuple.0);
}