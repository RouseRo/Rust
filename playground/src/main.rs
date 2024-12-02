// Everything is Rust is defauted to immutable
mod my_funcs;
mod other_funcs;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::subtract_10;

// use crate::my_funcs::subtract_10;

fn main() {
    let mut x: u32 = 50;
    println!("{}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    let z: u32 = subtract_10(x);
    println!("z is {}", z);

    x = 70;
    println!("x is {}", x);
}
