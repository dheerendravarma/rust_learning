use std::env;

fn factorial(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: factorial <number>");
        return;
    }

    let number: i64 = args[1].parse().unwrap();
    let result = factorial(number);
    println!("The factorial of {} is {}", number, result);
}