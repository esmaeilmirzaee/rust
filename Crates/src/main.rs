// it's possible to shorten a name
// use crate::archive::archiving::save_file as save;
use crate::archive::archiving::save_file;

// using library crates
use rand::Rng;

mod archive;
fn main() {
    println!("There're two types of crates: 1. binary 2. library!");
    save_file("somefile.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("A random number, {}", a);
}
