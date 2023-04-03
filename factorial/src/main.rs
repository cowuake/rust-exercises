use num_bigint::BigUint;
use std::env;
use std::io;
use std::io::Write;

// Easy peasy lemon squeezy
fn factorial(n: u32) -> BigUint {
    (1..=n).product()
}

fn main() {
    // The program can accept input from command line
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Parse the command line argument and compute its factorial
        // if the argument is of the correct type, exit otherwise
        let _number: u32 = match args[1].parse::<u32>() {
            Ok(n) => {
                println!("The factorial of {} is {}", n, factorial(n));
                return;
            }
            Err(_) => {
                eprintln!("Passed a non-valid argument to the program");
                return;
            }
        };
    } else {
        // Loop until a proper input is provided
        loop {
            // Needed every time since the type is recast during a loop iteration
            let mut input = String::new();

            // Ask user for input
            print!("Insert a number: ");
            io::stdout().flush().unwrap(); // If omitted, text may not appear

            // Read the user input
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            // Parse the given input and compute its factorial
            // if it's of the correct type, ask go on with the loop otherwise
            let _number: u32 = match input.trim().parse::<u32>() {
                Ok(n) => {
                    println!("\nThe factorial of {} is {}", n, factorial(n));
                    return;
                }
                Err(_) => {
                    eprintln!("\n======================================================");
                    eprintln!("Please enter a valid input (UNSIGNED INTEGER REQUIRED)");
                    eprintln!("======================================================");
                    println!(); // Let the terminal buffer breath
                    continue;
                }
            };
        } // loop
    }
}
