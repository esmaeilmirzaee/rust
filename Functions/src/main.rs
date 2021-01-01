fn main() {
    println!("Functions!");
    say_hi();

    let mut name = "Esmaeil";
    say_hello(name);
    goodbye(&mut name);
    println!("{}", name);


    println!("{}", asking_for_info("Esmaeil"));
}

fn say_hi() {
    println!("Hi");
}

///! pass by value
fn say_hello(name: &str) {
    println!("Hello {}", name);
}

///! pass by reference
fn goodbye(name: &mut &str) {
    println!("Goodbye {}", name);
    *name = "Sam";
}

///! returning a value
fn asking_for_info(name: &str) -> String {
    let greeting = format!("{} how old are you?", name);
    return greeting;
}

