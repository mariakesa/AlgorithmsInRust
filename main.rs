use std::io;

fn main() {
    println!("Enter number!");
    let mut a=String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    println!("A is: {a}");
}
