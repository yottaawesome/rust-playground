// mod tells the compiler to include stuff.rs as a module
mod stuff;
// use brings the function into scope so we can call it without stuff:: prefix
use stuff::superblah;

// zed  keymap file location on windows: ~/.config/zed/keymap.json
// A typical Rust executable (binary) project:
/*
   my_project/
   ├── Cargo.toml          # Project metadata & dependencies
   ├── Cargo.lock          # Locked dependency versions
   ├── src/
   │   ├── main.rs         # Entry point (fn main)
   │   ├── lib.rs          # Optional shared library root
   │   ├── module1.rs      # Top-level module
   │   └── module2/        # Module with submodules
   │       ├── mod.rs      # Module root (or use module2.rs above src/)
   │       └── sub.rs      # Submodule
   ├── tests/              # Integration tests
   │   └── integration.rs
   ├── benches/            # Benchmarks
   ├── examples/           # Example binaries
   │   └── demo.rs
   └── target/             # Build output (gitignored)

  Key points:
   - main.rs is the binary entry point
   - Modules are added via mod name; in the parent module
   - A module foo maps to either foo.rs or foo/mod.rs
   - lib.rs is optional — use it to share code between binaries or expose a library
   - Multiple binaries can live in src/bin/, each file becoming its own executable
*/

fn something(x: i32) -> i32 {
    println!("The value of x is: {x}");
    return x;
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let xx = tup.1; // sets xx to tup[1]
}

fn variables() {
    let x = 5; // immutable by default
    let mut y = 5; // mutable
    y = 15;
    const Something: i32 = 10;
}

fn arrays() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // 5 elements of value 3
}

fn control() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
}

fn looploop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // labelled loop
    'outer: loop {
        loop {
            break 'outer;
        }
    }

    println!("The result is {result}");
}

fn whileloop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn forloops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// moves
fn references1(str: String) {}

// references
fn references2(some: &String) {}

// references, only one reference can be active at a time
fn references3(some: &mut String) {}

fn main() {
    let literal: &str = "hello";
    let xx = String::from("hello");
    let aa = &xx;
    let bb = &xx;

    superblah();

    something(1);
}
