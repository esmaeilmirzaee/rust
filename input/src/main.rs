use std::io;

fn main() {
    //! This is main function.
    let mut input = String::new();
    println!("Say something");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}!", e);
        }
    }

}