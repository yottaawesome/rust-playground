use multi_bin::count_to;

/// A binary that counts up to a number provided as a CLI argument.
fn main() {
    let n: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    println!("Counting to {n}:");
    for num in count_to(n) {
        println!("  {num}");
    }
}
