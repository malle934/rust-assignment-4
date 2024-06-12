mod arith;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("CLI Calculator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Performs basic arithmetic operations")
        .arg(Arg::new("operation")
            .help("The operation to perform: add, sub, mul, div")
            .required(true)
            .index(1))
        .arg(Arg::new("a")
            .help("The first operand")
            .required(true)
            .index(2))
        .arg(Arg::new("b")
            .help("The second operand")
            .required(true)
            .index(3))
        .get_matches();

    let a: f64 = matches.get_one::<String>("a")
        .expect("required argument")
        .parse()
        .expect("Invalid number for operand a");
    let b: f64 = matches.get_one::<String>("b")
        .expect("required argument")
        .parse()
        .expect("Invalid number for operand b");

    let operation = matches.get_one::<String>("operation")
        .expect("required argument");

    match operation.as_str() {
        "add" | "+" => println!("{}", arith::add(a, b)),
        "sub" | "-" => println!("{}", arith::subtract(a, b)),
        "mul" | "*" => println!("{}", arith::multiply(a, b)),
        "div" | "/" => match arith::divide(a, b) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        },
        _ => eprintln!("Invalid operation. Use add, sub, mul, or div."),
    }
}
