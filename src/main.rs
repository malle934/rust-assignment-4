mod arithmetic;
use clap::{Arg, Command};
fn main() {
    let matches = Command::new("CLI Calculator")
        .arg(Arg::new("operation")
            .help("The operation to perform: add, subtract, multiply, divide")
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

        let a: f64 = match matches.get_one::<String>("operand1") {
            Some(val) => match val.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Invalid number for operand1.");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("Error: Operand1 is missing.");
                std::process::exit(1);
            }
        };
    
        let b: f64 = match matches.get_one::<String>("operand2") {
            Some(val) => match val.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Invalid number for operand2.");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("Error: Operand2 is missing.");
                std::process::exit(1);
            }
        };
    let operation = match matches.get_one::<String>("operation") {
        Some(op) => op,
        None => {
            eprintln!("Error: Operation not provided. Use -o to specify the operation.");
            std::process::exit(1);
        }
    };
    match operation.as_str() {
        "add" | "+" => println!("{}", arithmetic::add(a, b)),
        "subtract" | "-" => println!("{}", arithmetic::subtract(a, b)),
        "multiply" | "*" => println!("{}", arithmetic::multiply(a, b)),
        "divide" | "/"=> match arithmetic::divide(a, b) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        },
        _ => eprintln!("Invalid operation. Use add, subtract, multiply, or divide."),
    }
}
