fn main() {
    println!("Operators!");
    let a = 4 + 8;
    let b = 8 / 4;
    let c = 4 * 8;
    let d = 8 % 4;
    let e = 8 - 4;
    println!("{} {} {} {}", a, b, c, d);
    println!("{}", a >= b);
    /// Bitwise operators
    println!("{}", 10 & 3);
    println!("{}", 10 | 3);
    println!("{}", 10 ^ 3);
    println!("{}", 10 << 3);
    println!("{}", 10 >> 3);
}
