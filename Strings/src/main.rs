fn main() {
    println!("Strings!");
    /// String slices
    /// String slices are immutable
    let cat = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    /// string objects
    let mut dog = String::new();
    dog = "Max".to_string();

    let mut dog = String::from("Max");
    println!("{}", dog);

    format!("Hi I'm, {}, the owner of {}.", "Esmaeil", dog);
    println!("The length of {} is {}.", dog, dog.len());
    dog.push('\u{F601}');
    dog.push_str(" is a dog");
    println!("{}", dog);

    let mut new_dog = dog.replace("a", "my");
    println!("{}", new_dog);
}
