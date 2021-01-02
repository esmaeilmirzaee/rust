use rand::{Rng, thread_rng, random};
use rand::distributions::Alphanumeric; // to create random strings

fn main() {
    println!("Random!");

    let mut rng = rand::thread_rng();
    let mut random_number: i32 = rng.gen();
    println!("{}", random_number);

    random_number = rng.gen_range(0..100);
    println!("{}", random_number);

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect();
    println!("a random string: {}", random_string);
}
