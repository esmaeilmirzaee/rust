fn main() {
    println!("Arrays!");
    let primes = [2, 3, 5, 7, 11, 13, 17];
    let doubles: [f64; 4] = [2.8, 4.0, 101.1, 9.1];
    println!("{:?}", primes);
    println!("{:?}", doubles);

    let numbers = [0; 15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);

    numbers[1] = 10;
    println!("{:?}", numbers);

    for itr in numbers.iter() {
        println!("{}", itr);
    }
}
