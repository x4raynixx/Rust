use std::io;
use std::io::Write;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    print!("Enter #1 Number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num1).expect("Error reading input #1");

    print!("Enter #2 Number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num2).expect("Error reading input #2");

    let a: i32 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number #1");
            return;
        }
    };

    let b: i32 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number #2");
            return;
        }
    };

    println!("{a} + {b} = {}", a + b);
}
