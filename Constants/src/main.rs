fn main() {
    println!("Constants!");
    /// Constants are immutable
    /// Data types are mandatory and by convention the name is uppercase
    /// Shadowing is impossible.
    const URL: &str = "google.com";
    println!("{}", URL);
}
