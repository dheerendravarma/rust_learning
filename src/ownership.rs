fn print_string(s: String) {
    println!("The value of s is : {}", s);
}
fn main() {
    let s1 = String::from("Hello World");
    let s2 = s1;

    // println!("The value of s1 is : {}", s1);
    // println!("The value of s2 is : {}", s2);

    print_string(s2);
    print_string(s2);
}