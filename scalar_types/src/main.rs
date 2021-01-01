/// Four data types in Rust
fn main() {
    println!("Hello, world!");
    let million = 1_000_000;
    println!("{}", million);

    let is_day = true;
    let is_night = false;
    println!("{} {}", is_day, is_night);

    let char = 'A';
    let smiley_face = '\u{1F601}';
    println!("{} -> {}", char, smiley_face);
}
