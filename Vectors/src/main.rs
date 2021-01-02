fn main() {
    println!("Vectors!");
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("{:?}", primes);

    primes.push(7);
    println!("{:?}", primes);

    primes.remove(2);
    println!("{:?}", primes);

    let numbers = vec![2; 10];
    println!("{:?}", numbers);

    const DEFAULT: bool = true;
    let values = vec![DEFAULT; 8];
    println!("{:?}", values);

    let mut numbers = vec![2; 5];
    println!("{:?}", numbers);
    numbers[3] = 10;
    numbers[1] = 9;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}
