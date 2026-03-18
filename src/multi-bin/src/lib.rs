/// Shared greeting function used by multiple binaries.
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to multi-bin.", name)
}

/// Shared counter that counts from 1 to n, returning the results as a vector.
pub fn count_to(n: u32) -> Vec<u32> {
    (1..=n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "Hello, Rust! Welcome to multi-bin.");
    }

    #[test]
    fn test_count_to() {
        assert_eq!(count_to(5), vec![1, 2, 3, 4, 5]);
    }
}
