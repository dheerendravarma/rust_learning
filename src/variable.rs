fn main() {
    // Variables are immutable by default
    let mut x = 5;
    x = x + 1;
    println!("The value of x is : {}", x);

    let mut sum: i32 = 0;
    for idx in 0..10000000 {
        sum += idx;
    }
    println!("The sum is : {}", sum);

    let y: u8 = -100;
    println!("The value of y is : {}", y);

    // Signed Integer data types are ranging from i8 to i128
    // Un Signed Interger data types are ranging from u8 to u128
    // 
}