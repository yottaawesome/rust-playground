use multi_bin::greet;

/// A binary that greets each name provided as a CLI argument.
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("{}", greet("stranger"));
    } else {
        for name in &args {
            println!("{}", greet(name));
        }
    }
}
