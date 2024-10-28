use std::i32::MAX;

fn main() {
    // Variables are immutable by default
    let mut x = 5;
    x = x + 1;
    println!("The value of x is : {}", x);

    let mut sum_value: i64 = 0;
    for idx in 0..10000000 {
        sum_value += idx;
    }
    println!("The sum is : {}", sum_value); 

    let y: i8 = -100;
    println!("The value of y is : {}", y);

    // Signed Integer data types are ranging from i8 to i128
    // Un Signed Interger data types are ranging from u8 to u128
    // float data types are f32 and f64

    let pi = 3.14159;
    println!("The value of pi is : {}", pi);

    // print the max value of i32
    println!("The max value of i32 is : {}", MAX);

    // How to handle the variables with booleans
    let is_male: bool = true;
    let is_eligible_to_vote: bool = false;

    if is_eligible_to_vote {
        println!("You are eligible to vote");
    } else {
        println!("You are not eligible to vote");
    }

    if is_male {
        println("You are a Male");
    } else {
        println("You are not a Male");
    }
}