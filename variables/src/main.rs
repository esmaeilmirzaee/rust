/// Warnings won't be generated; To allow unused variables
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let pi = 3.141569;
    let name = "Esmaeil";
    let a_big_number : i64 = 1284832748932374829; // Error: Literal out of range for 'i32'

    println!("Hello, world!");

    /// Shadowing is allowed.
    let colour = "red";
    let colour = "blue";
    println!("{}",colour);

    /// Declaring multiple variables simultaneously.
    let (a, v, n) = (32, true, "Esmaeil");
    println!("{}{}{}",a, v, n);
}
