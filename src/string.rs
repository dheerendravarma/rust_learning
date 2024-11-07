fn main() {
    let s1 = String::from("Hello World");
    println!("The value of s1 is : {}", s1);

    let s2 = "Hello World";
    println!("The value of s2 is : {}", s2);

    // print the length of the string
    println!("The length of s2 is : {}", s2.len());

    // print the character by char from the string
    for ch in s2.chars() {
        println!("{}", ch);
    }

    // This will throw an run time exception
    // println!("character at 5ht index is :: {}", s2.chars().nth(11).unwrap());

    // This will not throw an run time exception
    match s2.chars().nth(11) {
        Some(ch) => println!("Character at 11th index is : {}", ch),
        None => println!("There is no character at 11th index"),
    }

    let s3: &str = "Hello World";
    let s4 = String::from("Hello World");

    println!("The value of s3 is : {}", s3);
    println!("The value of s4 is : {}", s4);

    // Print the type of the variable
    println!("The type of s3 is : {}", type_of(&s3));
}