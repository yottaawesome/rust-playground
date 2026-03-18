use multi_bin::{count_to, greet};

/// Default binary — demonstrates using shared library functions.
fn main() {
    let message = greet("World");
    println!("{message}");

    let numbers = count_to(5);
    println!("Counted: {numbers:?}");
}
