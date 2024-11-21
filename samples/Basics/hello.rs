
fn main() {
    println!("Hello, world!");

    let s = "hello";
    let t = &s; // borrow s as a reference
    println!("{}", t); // prints "hello"
}
